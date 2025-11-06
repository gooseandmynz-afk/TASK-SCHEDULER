use anyhow::Result;
use iced_task_scheduler::{load_tasks, save_tasks, Interval, Task};
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

    // Create config directory structure
    let config_path = temp_path
        .join("example")
        .join("task_scheduler_gui")
        .join("config");
    fs::create_dir_all(&config_path)?;

    Ok(temp_dir)
}

// Integration test for the full task lifecycle
#[tokio::test]
async fn test_task_lifecycle() -> Result<()> {
    let _temp_dir = setup_test_env().await?;

    // Explicitly clear all tasks first
    save_tasks(&[]).await?;

    // Start with no tasks - should be empty in clean environment
    let tasks = load_tasks().await?;
    assert!(
        tasks.is_empty(),
        "Should start with no tasks in clean environment"
    ); // Create and save some tasks
    let mut tasks = vec![
        Task::new("Task 1", Interval::Daily),
        Task::new("Task 2", Interval::Weekly),
    ];
    tasks[0].enabled = true;

    save_tasks(&tasks).await?;

    // Load and verify
    let loaded = load_tasks().await?;
    assert_eq!(loaded.len(), 2);
    assert_eq!(loaded[0].name, "Task 1");
    assert!(loaded[0].enabled);
    assert_eq!(loaded[1].name, "Task 2");
    assert!(!loaded[1].enabled);

    Ok(())
}

// Integration test for concurrent modifications
#[tokio::test]
async fn test_concurrent_modifications() -> Result<()> {
    let _temp_dir = setup_test_env().await?;

    // Ensure we start clean
    save_tasks(&[]).await?;

    // Initial tasks
    let tasks = vec![Task::new("Initial", Interval::Daily)];
    save_tasks(&tasks).await?;

    // Simulate concurrent modifications with delay
    let mut handles = Vec::new();

    for i in 0..3 {
        let task_name = format!("Task {}", i);
        let handle = tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(i * 10)).await;
            let mut tasks = load_tasks().await.unwrap();
            tasks.push(Task::new(task_name, Interval::Hourly));
            save_tasks(&tasks).await
        });
        handles.push(handle);
    }

    // Wait for all operations
    for handle in handles {
        handle.await??;
    }

    // Verify final state
    let final_tasks = load_tasks().await?;
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

    Ok(())
}
