// File: src/stats.rs
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub struct Stats {
    daily_pomodoros: Vec<(String, u32)>,       // (date, count)
    weekly_work_time: Vec<(String, Duration)>, // (week, total duration)
}

impl Stats {
    pub fn new() -> Self {
        Self {
            daily_pomodoros: Vec::new(),
            weekly_work_time: Vec::new(),
        }
    }

    pub fn save(&self) {
        if let Some(home_dir) = dirs::home_dir() {
            let backup_dir = home_dir.join(".rust_pomodoro_backup");
            fs::create_dir_all(&backup_dir).unwrap_or(());
            let stats_json = serde_json::to_string(&self).unwrap_or_default();
            fs::write(backup_dir.join("stats.json"), stats_json).unwrap_or(());
        }
    }

    pub fn load() -> Self {
        if let Some(home_dir) = dirs::home_dir() {
            let file_path = home_dir.join(".rust_pomodoro_backup/stats.json");
            if Path::new(&file_path).exists() {
                if let Ok(contents) = fs::read_to_string(file_path) {
                    if let Ok(stats) = serde_json::from_str(&contents) {
                        return stats;
                    }
                }
            }
        }
        Self::new()
    }

    pub fn add_pomodoro(&mut self) {
        let today = Utc::now().format("%Y-%m-%d").to_string();
        if let Some((_, count)) = self
            .daily_pomodoros
            .iter_mut()
            .find(|(date, _)| *date == today)
        {
            *count += 1;
        } else {
            self.daily_pomodoros.push((today, 1));
        }
    }

    pub fn add_work_time(&mut self, duration: Duration) {
        let week = Utc::now().format("%Y-W%V").to_string(); // ISO week
        if let Some((_, total)) = self.weekly_work_time.iter_mut().find(|(w, _)| *w == week) {
            *total += duration;
        } else {
            self.weekly_work_time.push((week, duration));
        }
    }
}
