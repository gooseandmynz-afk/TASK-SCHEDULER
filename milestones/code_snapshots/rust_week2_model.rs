// Week 2: Data model and types
// Introduces Task and Interval with serde support for persistence
// See `week2_ARCHITECTURE.md` for architecture notes and spike checklist.

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

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
}

// This module will be used by later weeks (persistence, CLI, GUI)
