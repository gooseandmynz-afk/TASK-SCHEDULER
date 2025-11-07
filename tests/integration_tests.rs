use anyhow::Result;
use iced_task_scheduler::{load_tasks, save_tasks, Interval, Task};
use serial_test::serial;
use std::env;
use std::fs;
use tempfile::TempDir;

async fn setup_test_env() -> Result<TempDir> {
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path().to_path_buf();

    // Set up environment to use our temp directory
    env::set_var("APPDATA", &temp_path);
    env::set_var("HOME", &temp_path);
    env::set_var("XDG_CONFIG_HOME", &temp_path);

    // Also set an explicit override for the library so tests are fully isolated
    let override_dir = temp_path.join("config_override");
    fs::create_dir_all(&override_dir)?;
    env::set_var("TASK_SCHEDULER_CONFIG_DIR", &override_dir);

    // Create config directory structure
    let config_path = temp_path
        .join("example")
        .join("task_scheduler_gui")
        .join("config");
    fs::create_dir_all(&config_path)?;

    Ok(temp_dir)
}

// Integration test for the full task lifecycle
#[serial]
#[test]
fn test_task_lifecycle() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let _temp_dir = setup_test_env().await.unwrap();

        // Explicitly clear all tasks first
        save_tasks(&[]).await.unwrap();

        // Start with no tasks - should be empty in clean environment
        let tasks = load_tasks().await.unwrap();
        assert!(
            tasks.is_empty(),
            "Should start with no tasks in clean environment"
        );

        // Create and save some tasks
        let mut tasks = vec![
            Task::new("Task 1", Interval::Daily),
            Task::new("Task 2", Interval::Weekly),
        ];
        tasks[0].enabled = true;

        save_tasks(&tasks).await.unwrap();

        // Load and verify
        let loaded = load_tasks().await.unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].name, "Task 1");
        assert!(loaded[0].enabled);
        assert_eq!(loaded[1].name, "Task 2");
        assert!(!loaded[1].enabled);
    });
}

// Integration test for concurrent modifications
#[serial]
#[test]
fn test_concurrent_modifications() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let _temp_dir = setup_test_env().await.unwrap();

        // Ensure we start clean
        save_tasks(&[]).await.unwrap();

        // Initial tasks
        let tasks = vec![Task::new("Initial", Interval::Daily)];
        save_tasks(&tasks).await.unwrap();

        // Simulate concurrent modifications with delay
        let mut handles = Vec::new();

        for i in 0..3 {
            let task_name = format!("Task {}", i);
            let handle = tokio::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(i * 10)).await;
                let mut tasks = load_tasks().await.unwrap();
                tasks.push(Task::new(task_name, Interval::Hourly));
                save_tasks(&tasks).await.unwrap();
            });
            handles.push(handle);
        }

        // Wait for all operations
        for handle in handles {
            handle.await.unwrap();
        }

        // Verify final state
        let final_tasks = load_tasks().await.unwrap();
        assert!(final_tasks.len() > 1, "Should have added tasks");
        assert!(
            final_tasks.iter().any(|t| t.name == "Initial"),
            "Original task should exist"
        );

        // Verify all tasks were added
        for i in 0..3 {
            assert!(
                final_tasks.iter().any(|t| t.name == format!("Task {}", i)),
                "Task {} should exist",
                i
            );
        }
    });
}
