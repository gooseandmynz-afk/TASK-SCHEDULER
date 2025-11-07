// Week 5: Scheduler core
// Adds a function to compute which tasks should run now and a small runner example
// See `week5_PERSISTENCE.md` for persistence polish, settings and progress indicators.

use chrono::Local;

mod rust_week2_model;
mod rust_week3_persistence;

use rust_week2_model::Task;
use rust_week3_persistence::{load_tasks, save_tasks};

fn tasks_to_run(tasks: &[Task]) -> Vec<usize> {
    let now = Local::now();
    let mut out = Vec::new();
    for (i, t) in tasks.iter().enumerate() {
        if !t.enabled {
            continue;
        }
        let should = if let Some(last) = t.last_run {
            match t.interval {
                rust_week2_model::Interval::Hourly => {
                    now.signed_duration_since(last).num_hours() >= 1
                }
                rust_week2_model::Interval::Daily => {
                    now.signed_duration_since(last).num_days() >= 1
                }
                rust_week2_model::Interval::Weekly => {
                    now.signed_duration_since(last).num_weeks() >= 1
                }
            }
        } else {
            true
        };
        if should {
            out.push(i);
        }
    }
    out
}

fn main() {
    let mut tasks = load_tasks().unwrap_or_default();
    let to_run = tasks_to_run(&tasks);
    println!("Tasks to run now: {:?}", to_run);
    for idx in to_run {
        // mark complete (simulate running)
        tasks[idx].last_run = Some(Local::now());
        println!("Simulated run: {}", tasks[idx].name);
    }
    let _ = save_tasks(&tasks);
}
