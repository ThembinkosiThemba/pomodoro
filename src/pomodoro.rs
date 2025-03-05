// File: src/pomodoro.rs
use crate::sound::play_notification;
use eframe::egui;
use egui::{Align2, Color32, FontId, RichText, Sense, Stroke, Ui, Vec2};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, time::Duration};

#[derive(PartialEq, Copy, Clone)]
pub enum PomodoroState {
    Stopped,
    Work,
    ShortBreak,
    LongBreak,
    Paused,
}

impl PomodoroState {
    fn label(&self) -> &'static str {
        match self {
            PomodoroState::Stopped => "Stopped",
            PomodoroState::Work => "Work",
            PomodoroState::ShortBreak => "Short Break",
            PomodoroState::LongBreak => "Long Break",
            PomodoroState::Paused => "Paused",
        }
    }

    fn color(&self) -> Color32 {
        match self {
            PomodoroState::Stopped => Color32::GRAY,
            PomodoroState::Work => Color32::from_rgb(235, 87, 87), // Red for work
            PomodoroState::ShortBreak => Color32::from_rgb(106, 176, 76), // Green for short break
            PomodoroState::LongBreak => Color32::from_rgb(79, 134, 198), // Blue for long break
            PomodoroState::Paused => Color32::GRAY,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Metrics {
    pub completed_pomodoros: u32,
    total_work_time: Duration,
}

pub struct Pomodoro {
    pub state: PomodoroState,
    pub elapsed: Duration,
    pub work_duration: Duration,
    short_break_duration: Duration,
    long_break_duration: Duration,
    cycles_before_long_break: u32,
    completed_cycles: u32,
    show_notification: bool,
    total_work_time: Duration,
    completed_pomodoros: u32,
    pub metrics: Metrics,
    previous_state: Option<PomodoroState>,
}

impl Pomodoro {
    pub fn new() -> Self {
        let metrics = Self::load_metrics().unwrap_or(Metrics {
            completed_pomodoros: 0,
            total_work_time: Duration::from_secs(0),
        });

        Self {
            state: PomodoroState::Stopped,
            elapsed: Duration::from_secs(0),
            work_duration: Duration::from_secs(25 * 60), // 25 minutes
            short_break_duration: Duration::from_secs(5 * 60), // 5 minutes
            long_break_duration: Duration::from_secs(15 * 60), // 15 minutes
            cycles_before_long_break: 4,
            completed_cycles: 0,
            show_notification: false,
            total_work_time: metrics.total_work_time,
            completed_pomodoros: metrics.completed_pomodoros,
            metrics,
            previous_state: None,
        }
    }

    fn save_metrics(&self) {
        if let Some(home_dir) = dirs::home_dir() {
            let backup_dir = home_dir.join(".rust_pomodoro_backup");
            fs::create_dir_all(&backup_dir).unwrap_or(());
            let metrics_json = serde_json::to_string(&self.metrics).unwrap_or_default();
            fs::write(backup_dir.join("metrics.json"), metrics_json).unwrap_or(());
        }
    }

    fn load_metrics() -> Option<Metrics> {
        if let Some(home_dir) = dirs::home_dir() {
            let file_path = home_dir.join(".rust_pomodoro_backup/metrics.json");
            if Path::new(&file_path).exists() {
                if let Ok(contents) = fs::read_to_string(file_path) {
                    if let Ok(metrics) = serde_json::from_str(&contents) {
                        return Some(metrics);
                    }
                }
            }
        }
        None
    }

    pub fn update(&mut self, elapsed: Duration, ctx: &egui::Context) {
        if self.state == PomodoroState::Stopped || self.state == PomodoroState::Paused {
            return; // we do not update elapsed time when stopped or paused
        }

        // Add elapsed time
        self.elapsed += elapsed;

        if self.state == PomodoroState::Work {
            self.total_work_time += elapsed;
        }

        // Check if the current interval is complete
        let current_duration = self.current_duration();
        if self.elapsed >= current_duration {
            let next_state = match self.state {
                PomodoroState::Work => {
                    self.completed_cycles += 1;
                    self.metrics.completed_pomodoros += 1;
                    self.metrics.total_work_time += current_duration;
                    if self.completed_cycles % self.cycles_before_long_break == 0 {
                        PomodoroState::LongBreak
                    } else {
                        PomodoroState::ShortBreak
                    }
                }
                PomodoroState::ShortBreak | PomodoroState::LongBreak => PomodoroState::Work,
                PomodoroState::Stopped | PomodoroState::Paused => unreachable!(),
            };

            // Reset elapsed time
            self.elapsed = Duration::from_secs(0);

            // Update state
            self.state = next_state;

            // Play notification sound
            play_notification();

            // Show notification
            self.show_notification = true;

            // Request a repaint to show the notification immediately
            ctx.request_repaint();

            self.save_metrics();
        } else if self.state == PomodoroState::Work {
            self.metrics.total_work_time += elapsed;
        }
    }

    fn current_duration(&self) -> Duration {
        match self.state {
            PomodoroState::Stopped => Duration::from_secs(0),
            PomodoroState::Work => self.work_duration,
            PomodoroState::ShortBreak => self.short_break_duration,
            PomodoroState::LongBreak => self.long_break_duration,
            PomodoroState::Paused => {
                // Return the duration of the previous state
                match self.previous_state {
                    Some(PomodoroState::Work) => self.work_duration,
                    Some(PomodoroState::ShortBreak) => self.short_break_duration,
                    Some(PomodoroState::LongBreak) => self.long_break_duration,
                    _ => Duration::from_secs(0), // Fallback if no previous state
                }
            }
        }
    }

    fn remaining_time(&self) -> Duration {
        let current_duration = self.current_duration();
        if self.elapsed > current_duration {
            Duration::from_secs(0)
        } else {
            current_duration - self.elapsed
        }
    }

    fn format_time(duration: Duration) -> String {
        let total_seconds = duration.as_secs();
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }

    fn progress(&self) -> f32 {
        if self.state == PomodoroState::Stopped || self.state == PomodoroState::Paused {
            return 0.0; // No progress while stopped or paused
        }

        let current_duration = self.current_duration();
        if current_duration.as_secs_f32() == 0.0 {
            return 0.0;
        }

        self.elapsed.as_secs_f32() / current_duration.as_secs_f32()
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.heading("Pomodoro Timer");
            ui.add_space(20.0);

            // Timer display
            let remaining = Self::format_time(self.remaining_time());
            let color = self.state.color();

            // Display current state
            ui.label(
                RichText::new(self.state.label())
                    .font(FontId::proportional(18.0))
                    .color(color),
            );

            // Timer circle
            let timer_size = Vec2::splat(ui.available_width().min(240.0));
            let (response, painter) = ui.allocate_painter(timer_size, Sense::hover());
            let center = response.rect.center();
            let radius = response.rect.width() / 2.0 - 10.0;

            // Draw progress circle
            if self.state != PomodoroState::Stopped {
                let progress = self.progress();
                let angle = std::f32::consts::TAU * progress - std::f32::consts::FRAC_PI_2;

                // Background circle
                painter.circle_stroke(center, radius, Stroke::new(5.0, Color32::DARK_GRAY));

                // Progress arc (we'll approximate with line segments)
                let segments = 100;
                let mut last_point = egui::Pos2::new(
                    center.x + radius * (-std::f32::consts::FRAC_PI_2).cos(),
                    center.y + radius * (-std::f32::consts::FRAC_PI_2).sin(),
                );

                for i in 1..=((segments as f32) * progress).ceil() as usize {
                    let segment_angle = std::f32::consts::TAU * (i as f32 / segments as f32)
                        - std::f32::consts::FRAC_PI_2;
                    let end_angle = if i as f32 / segments as f32 > progress {
                        angle
                    } else {
                        segment_angle
                    };

                    let point = egui::Pos2::new(
                        center.x + radius * end_angle.cos(),
                        center.y + radius * end_angle.sin(),
                    );

                    painter.line_segment([last_point, point], Stroke::new(5.0, color));
                    last_point = point;
                }
            }

            // Draw time text
            painter.text(
                center,
                Align2::CENTER_CENTER,
                remaining,
                FontId::proportional(32.0),
                if self.state == PomodoroState::Stopped {
                    Color32::GRAY
                } else {
                    Color32::WHITE
                },
            );

            ui.add_space(30.0);

            // Controls
            ui.horizontal(|ui| {
                let button_text = if self.state == PomodoroState::Stopped {
                    "Start"
                } else {
                    "Stop"
                };

                if ui.button(button_text).clicked() {
                    if self.state == PomodoroState::Stopped {
                        self.state = PomodoroState::Work;
                        self.elapsed = Duration::from_secs(0);
                        self.previous_state = None; // Reset previous state on new start
                    } else {
                        self.state = PomodoroState::Stopped;
                        self.previous_state = None; // Reset previous state on stop
                    }
                }

                if self.state != PomodoroState::Stopped && self.state != PomodoroState::Paused {
                    if ui.button("Pause").clicked() {
                        self.state = PomodoroState::Paused;
                        self.previous_state = Some(self.state); // Store the current state before pausing
                    }
                }

                if self.state == PomodoroState::Paused {
                    if ui.button("Resume").clicked() {
                        self.state = self.previous_state.unwrap_or(PomodoroState::Work); // Resume to previous state
                        self.previous_state = None; // Clear previous state after resuming
                    }
                }

                if self.state != PomodoroState::Stopped {
                    if ui.button("Reset").clicked() {
                        self.elapsed = Duration::from_secs(0);
                        if self.state == PomodoroState::Paused {
                            self.state = self.previous_state.unwrap_or(PomodoroState::Work); // Resume if reset while paused
                            self.previous_state = None;
                        }
                    }
                }
            });

            ui.add_space(20.0);

            // Settings
            ui.collapsing("Settings", |ui| {
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.label("Work duration (min):");
                    let mut work_mins = self.work_duration.as_secs() / 60;
                    if ui
                        .add(egui::DragValue::new(&mut work_mins).range(1..=60))
                        .changed()
                    {
                        self.work_duration = Duration::from_secs(work_mins * 60);
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Short break (min):");
                    let mut short_break_mins = self.short_break_duration.as_secs() / 60;
                    if ui
                        .add(egui::DragValue::new(&mut short_break_mins).range(1..=30))
                        .changed()
                    {
                        self.short_break_duration = Duration::from_secs(short_break_mins * 60);
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Long break (min):");
                    let mut long_break_mins = self.long_break_duration.as_secs() / 60;
                    if ui
                        .add(egui::DragValue::new(&mut long_break_mins).range(5..=60))
                        .changed()
                    {
                        self.long_break_duration = Duration::from_secs(long_break_mins * 60);
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Cycles before long break:");
                    if ui
                        .add(egui::DragValue::new(&mut self.cycles_before_long_break).range(1..=10))
                        .changed()
                    {}
                });
            });

            ui.collapsing("Metrics", |ui| {
                ui.label(format!(
                    "Completed Pomodoros: {}",
                    self.metrics.completed_pomodoros
                ));
                let hours = self.metrics.total_work_time.as_secs() / 3600;
                let minutes = (self.metrics.total_work_time.as_secs() % 3600) / 60;
                ui.label(format!("Total Work Time: {}h {}m", hours, minutes));
                let efficiency = if self.completed_cycles > 0 {
                    self.metrics.completed_pomodoros as f32 / self.completed_cycles as f32
                } else {
                    0.0
                };
                ui.label(format!("Efficiency: {:.1} pomodoros/cycle", efficiency));
            });

            // Notification
            if self.show_notification {
                ui.add_space(10.0);
                let text = match self.state {
                    PomodoroState::Work => "Time to focus! Work session started.",
                    PomodoroState::ShortBreak => "Take a short break!",
                    PomodoroState::LongBreak => "Time for a longer break. Well done!",
                    PomodoroState::Stopped => "",
                    PomodoroState::Paused => "",
                };

                ui.colored_label(self.state.color(), text);

                // Clear notification after a few seconds
                if self.elapsed.as_secs() >= 3 {
                    self.show_notification = false;
                }
            }
        });
    }
}
