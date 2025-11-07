// Week 3: Persistence helpers
// Implements saving/loading tasks to a project config path (JSON)
// See `week3_DATA.md` for data-layer design, testing notes and logging guidance.

use crate::rust_week2_model::Task;
use anyhow::{anyhow, Context};
use directories::ProjectDirs;
use serde_json;
use std::fs;
use std::path::PathBuf;

fn project_file_path() -> Option<PathBuf> {
    ProjectDirs::from("com", "example", "task_scheduler_cli").map(|dirs| {
        let dir = dirs.config_dir();
        let _ = fs::create_dir_all(dir);
        dir.join("tasks.json")
    })
}

pub fn save_tasks(tasks: &[Task]) -> Result<(), anyhow::Error> {
    let path = project_file_path().ok_or_else(|| anyhow!("no config dir"))?;
    let json = serde_json::to_string_pretty(tasks).context("serializing tasks")?;
    let mut tmp = path.clone();
    tmp.set_extension("json.tmp");
    fs::write(&tmp, &json).with_context(|| format!("writing temp file {}", tmp.display()))?;
    fs::rename(&tmp, &path)
        .with_context(|| format!("renaming {} -> {}", tmp.display(), path.display()))?;
    Ok(())
}

pub fn load_tasks() -> Result<Vec<Task>, anyhow::Error> {
    let path = project_file_path().ok_or_else(|| anyhow!("no config dir"))?;
    if !path.exists() {
        return Ok(vec![]);
    }
    let data = fs::read_to_string(&path)
        .with_context(|| format!("reading tasks file {}", path.display()))?;
    let list: Vec<Task> = serde_json::from_str(&data).context("parsing tasks JSON")?;
    Ok(list)
}
