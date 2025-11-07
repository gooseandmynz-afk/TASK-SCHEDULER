// Week 4: Simple CLI
// Adds ability to list/add tasks via a tiny text-menu (progress toward full app)
// See `week4_UI.md` for UI design, table view, detail pane and delete flow.

use chrono::Local;
use std::io::{self, Write};

mod rust_week2_model;
mod rust_week3_persistence;

use rust_week2_model::{Interval, Task};
use rust_week3_persistence::{load_tasks, save_tasks};

fn prompt(s: &str) -> String {
    print!("{}: ", s);
    let _ = io::stdout().flush();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok();
    buf.trim().to_string()
}

fn main() {
    println!("Rust Task Scheduler — Week 4 (CLI)");
    loop {
        println!("\nCommands: list, add, toggle <idx>, run-check, quit");
        let cmd = prompt("cmd");
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "list" => match load_tasks() {
                Ok(ts) => {
                    for (i, t) in ts.iter().enumerate() {
                        println!(
                            "[{}] {} ({}), enabled={} last_run={:?}",
                            i, t.name, t.interval, t.enabled, t.last_run
                        );
                    }
                }
                Err(e) => println!("err: {}", e),
            },
            "add" => {
                let name = prompt("Task name");
                let interval = prompt("Interval (hourly/daily/weekly)");
                let iv = match interval.to_lowercase().as_str() {
                    "hourly" => Interval::Hourly,
                    "weekly" => Interval::Weekly,
                    _ => Interval::Daily,
                };
                let mut tasks = load_tasks().unwrap_or_default();
                tasks.push(Task::new(name, iv));
                if let Err(e) = save_tasks(&tasks) {
                    println!("save err: {}", e);
                } else {
                    println!("added");
                }
            }
            "toggle" => {
                if parts.len() >= 2 {
                    if let Ok(idx) = parts[1].parse::<usize>() {
                        let mut tasks = load_tasks().unwrap_or_default();
                        if idx < tasks.len() {
                            tasks[idx].enabled = !tasks[idx].enabled;
                            save_tasks(&tasks).ok();
                        }
                    }
                }
            }
            "run-check" => {
                // For week 4 we just print tasks that would run now
                let tasks = load_tasks().unwrap_or_default();
                let now = Local::now();
                for (i, t) in tasks.iter().enumerate() {
                    if t.enabled {
                        let should = if let Some(last) = t.last_run {
                            now.signed_duration_since(last).num_hours() >= 1
                        } else {
                            true
                        };
                        if should {
                            println!("Would run: [{}] {}", i, t.name);
                        }
                    }
                }
            }
            "quit" => break,
            _ => println!("unknown command"),
        }
    }
}
