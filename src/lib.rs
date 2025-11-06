use anyhow::{anyhow, Context};
use chrono::{DateTime, Local};
use directories::ProjectDirs;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Interval {
    Hourly,
    Daily,
    Weekly,
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Interval::Hourly => write!(f, "Hourly"),
            Interval::Daily => write!(f, "Daily"),
            Interval::Weekly => write!(f, "Weekly"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub interval: Interval,
    pub last_run: Option<DateTime<Local>>,
    pub enabled: bool,
}

impl Task {
    pub fn new(name: impl Into<String>, interval: Interval) -> Self {
        Self {
            name: name.into(),
            interval,
            last_run: None,
            enabled: false,
        }
    }

    pub fn should_run(&self, now: DateTime<Local>) -> bool {
        if !self.enabled {
            return false;
        }

        if let Some(last) = self.last_run {
            match self.interval {
                Interval::Hourly => now.signed_duration_since(last).num_hours() >= 1,
                Interval::Daily => now.signed_duration_since(last).num_days() >= 1,
                Interval::Weekly => now.signed_duration_since(last).num_weeks() >= 1,
            }
        } else {
            true // Never run before
        }
    }

    pub fn mark_complete(&mut self) {
        self.last_run = Some(Local::now());
    }
}

pub fn project_file_path() -> Option<PathBuf> {
    ProjectDirs::from("com", "example", "task_scheduler_gui").map(|dirs| {
        let dir = dirs.config_dir();
        let _ = fs::create_dir_all(dir);
        dir.join("tasks.json")
    })
}

pub async fn save_tasks(tasks: &[Task]) -> Result<(), anyhow::Error> {
    let path = project_file_path().ok_or_else(|| anyhow!("no config dir"))?;

    // If tasks is empty and we're asked to save, clear all tasks
    if tasks.is_empty() {
        if path.exists() {
            fs::remove_file(&path)?;
        }
        return Ok(());
    }

    // Load current tasks with retry for concurrent access
    let max_retries = 3;
    let mut retry_count = 0;
    let current_tasks = loop {
        if retry_count >= max_retries {
            return Err(anyhow!(
                "Failed to save tasks after {} retries",
                max_retries
            ));
        }

        let current = if path.exists() {
            match load_tasks().await {
                Ok(tasks) => tasks,
                Err(e) => {
                    if retry_count < max_retries - 1 {
                        retry_count += 1;
                        tokio::time::sleep(std::time::Duration::from_millis(
                            10 * (1 << retry_count),
                        ))
                        .await;
                        continue;
                    } else {
                        return Err(e);
                    }
                }
            }
        } else {
            vec![]
        };
        break current;
    };

    // Create a merged set of tasks, preserving order
    let mut merged_tasks = current_tasks;
    for task in tasks {
        if let Some(existing) = merged_tasks.iter_mut().find(|t| t.name == task.name) {
            *existing = task.clone();
        } else {
            merged_tasks.push(task.clone());
        }
    }

    let json = serde_json::to_string_pretty(&merged_tasks).context("serializing tasks")?;

    // Use a unique temp filename to avoid conflicts in concurrent writes
    let mut rng = rand::thread_rng();
    let tmp = path.with_extension(format!("tmp.{}", rng.gen::<u64>()));

    // Write to temp file first
    fs::write(&tmp, &json).with_context(|| format!("writing temp file {}", tmp.display()))?;

    // Use atomic rename for safe concurrent access
    if let Err(e) = fs::rename(&tmp, &path) {
        // Clean up temp file on error
        let _ = fs::remove_file(&tmp);
        return Err(e).with_context(|| {
            format!("renaming temp file {} to {}", tmp.display(), path.display())
        });
    }

    // Remove any stray temp files that might be left from failed previous attempts
    if let Some(parent) = path.parent() {
        if let Ok(entries) = fs::read_dir(parent) {
            for entry in entries.flatten() {
                if let Ok(name) = entry.file_name().into_string() {
                    if name.starts_with(&format!(
                        "{}.tmp.",
                        path.file_name().unwrap().to_string_lossy()
                    )) {
                        let _ = fs::remove_file(entry.path());
                    }
                }
            }
        }
    }

    Ok(())
}

pub async fn load_tasks() -> Result<Vec<Task>, anyhow::Error> {
    let path = project_file_path().ok_or_else(|| anyhow!("no config dir"))?;
    if !path.exists() {
        return Ok(vec![]);
    }

    let data = fs::read_to_string(&path)
        .with_context(|| format!("reading tasks file {}", path.display()))?;

    // Try direct deserialization first
    if let Ok(list) = serde_json::from_str::<Vec<Task>>(&data) {
        return Ok(list);
    }

    // Fallback: manual parsing for backward compatibility
    let v: serde_json::Value = serde_json::from_str(&data).context("parsing tasks JSON")?;
    let arr = v.as_array().ok_or_else(|| anyhow!("expected array"))?;

    let mut out = Vec::with_capacity(arr.len());
    for item in arr {
        let name = item
            .get("name")
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string();

        let interval = match item.get("interval").and_then(|s| s.as_str()) {
            Some("Hourly") | Some("hourly") => Interval::Hourly,
            Some("Weekly") | Some("weekly") => Interval::Weekly,
            _ => Interval::Daily,
        };

        let enabled = item
            .get("enabled")
            .and_then(|b| b.as_bool())
            .unwrap_or(false);

        let last_run = item.get("last_run").and_then(|lr| {
            if lr.is_null() {
                None
            } else if let Some(s) = lr.as_str() {
                DateTime::parse_from_rfc3339(s)
                    .ok()
                    .map(|dt| dt.with_timezone(&Local))
            } else {
                None
            }
        });

        out.push(Task {
            name,
            interval,
            last_run,
            enabled,
        });
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_interval_display() {
        assert_eq!(Interval::Hourly.to_string(), "Hourly");
        assert_eq!(Interval::Daily.to_string(), "Daily");
        assert_eq!(Interval::Weekly.to_string(), "Weekly");
    }

    #[test]
    fn test_task_creation() {
        let task = Task::new("Test Task", Interval::Daily);
        assert_eq!(task.name, "Test Task");
        assert_eq!(task.interval, Interval::Daily);
        assert!(!task.enabled);
        assert!(task.last_run.is_none());
    }

    #[test]
    fn test_task_should_run() {
        let now = Local::now();

        let mut task = Task::new("Test", Interval::Hourly);
        assert!(!task.should_run(now)); // Disabled tasks don't run

        task.enabled = true;
        assert!(task.should_run(now)); // No last_run means should run

        task.last_run = Some(now - Duration::hours(2));
        assert!(task.should_run(now)); // 2 hours ago > 1 hour interval

        task.last_run = Some(now - Duration::minutes(30));
        assert!(!task.should_run(now)); // 30 mins ago < 1 hour interval
    }

    #[test]
    fn test_daily_interval() {
        let now = Local::now();
        let mut task = Task::new("Daily", Interval::Daily);
        task.enabled = true;

        task.last_run = Some(now - Duration::days(2));
        assert!(task.should_run(now));

        task.last_run = Some(now - Duration::hours(23));
        assert!(!task.should_run(now));
    }

    #[test]
    fn test_weekly_interval() {
        let now = Local::now();
        let mut task = Task::new("Weekly", Interval::Weekly);
        task.enabled = true;

        task.last_run = Some(now - Duration::weeks(2));
        assert!(task.should_run(now));

        task.last_run = Some(now - Duration::days(6));
        assert!(!task.should_run(now));
    }

    #[test]
    fn test_mark_complete() {
        let mut task = Task::new("Test", Interval::Daily);
        assert!(task.last_run.is_none());

        task.mark_complete();
        assert!(task.last_run.is_some());

        let time_diff = Local::now()
            .signed_duration_since(task.last_run.unwrap())
            .num_seconds();
        assert!(time_diff < 1); // Marked just now
    }

    #[test]
    fn test_project_path() {
        if let Some(path) = project_file_path() {
            assert!(path.ends_with("tasks.json"));
            assert!(path.to_string_lossy().contains("task_scheduler_gui"));
        }
    }

    // This test demonstrates fixing a bug where task enabled state wasn't preserved
    #[test]
    fn test_task_enabled_serialization() {
        let mut task = Task::new("Test", Interval::Daily);
        task.enabled = true;

        let json = serde_json::to_string(&task).unwrap();
        let decoded: Task = serde_json::from_str(&json).unwrap();

        assert!(
            decoded.enabled,
            "Task enabled state should be preserved after serialization"
        );
    }

    #[test]
    fn test_empty_task_name_handling() {
        let task = Task::new("", Interval::Daily);
        assert_eq!(task.name, "", "Empty task names should be allowed");

        let json = serde_json::to_string(&task).unwrap();
        let decoded: Task = serde_json::from_str(&json).unwrap();
        assert_eq!(
            decoded.name, "",
            "Empty names should round-trip through JSON"
        );
    }

    #[test]
    fn test_future_last_run() {
        let now = Local::now();
        let mut task = Task::new("Future", Interval::Daily);
        task.enabled = true;
        task.last_run = Some(now + Duration::days(1));

        assert!(
            !task.should_run(now),
            "Tasks with future last_run should not run"
        );
    }
}
