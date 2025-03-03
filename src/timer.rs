// File: src/timer.rs
use crate::sound::play_alarm;
use eframe::egui;
use egui::{Align2, Color32, FontId, RichText, Stroke, Ui, Vec2};
use std::time::Duration;

#[derive(PartialEq)]
pub enum TimerState {
    Stopped,
    Running,
    Paused,
    Completed,
}

pub struct Timer {
    state: TimerState,
    elapsed: Duration,
    duration: Duration,
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            state: TimerState::Stopped,
            elapsed: Duration::from_secs(0),
            duration: Duration::from_secs(0),
            hours: 0,
            minutes: 5,
            seconds: 0,
        }
    }

    pub fn update(&mut self, elapsed: Duration, ctx: &egui::Context) {
        if self.state != TimerState::Running {
            return;
        }

        self.elapsed += elapsed;

        if self.elapsed >= self.duration {
            self.state = TimerState::Completed;
            play_alarm();
            ctx.request_repaint();
        }
    }

    fn remaining_time(&self) -> Duration {
        if self.elapsed > self.duration {
            Duration::from_secs(0)
        } else {
            self.duration - self.elapsed
        }
    }

    fn format_time(duration: Duration) -> String {
        let total_seconds = duration.as_secs();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        if hours > 0 {
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        } else {
            format!("{:02}:{:02}", minutes, seconds)
        }
    }

    fn progress(&self) -> f32 {
        if self.state == TimerState::Stopped || self.duration.as_secs_f32() == 0.0 {
            return 0.0;
        }
        self.elapsed.as_secs_f32() / self.duration.as_secs_f32()
    }

    fn set_duration(&mut self) {
        let total_seconds =
            (self.hours as u64 * 3600) + (self.minutes as u64 * 60) + self.seconds as u64;
        self.duration = Duration::from_secs(total_seconds);
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.heading("Custom Timer");
            ui.add_space(20.0);

            match self.state {
                TimerState::Stopped => {
                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        ui.add_space(ui.available_width() * 0.1);

                        ui.vertical(|ui| {
                            ui.label("Hours");
                            ui.add(
                                egui::DragValue::new(&mut self.hours)
                                    .range(0..=23)
                                    .speed(0.1),
                            );
                        });

                        ui.label(RichText::new(":").font(FontId::proportional(24.0)));

                        ui.vertical(|ui| {
                            ui.label("Minutes");
                            ui.add(
                                egui::DragValue::new(&mut self.minutes)
                                    .range(0..=59)
                                    .speed(0.1),
                            );
                        });

                        ui.label(RichText::new(":").font(FontId::proportional(24.0)));

                        ui.vertical(|ui| {
                            ui.label("Seconds");
                            ui.add(
                                egui::DragValue::new(&mut self.seconds)
                                    .range(0..=59)
                                    .speed(0.1),
                            );
                        });

                        ui.add_space(ui.available_width() * 0.1);
                    });

                    ui.add_space(30.0);

                    ui.horizontal(|ui| {
                        if ui.button("1 min").clicked() {
                            self.hours = 0;
                            self.minutes = 1;
                            self.seconds = 0;
                        }
                        if ui.button("5 min").clicked() {
                            self.hours = 0;
                            self.minutes = 5;
                            self.seconds = 0;
                        }
                        if ui.button("10 min").clicked() {
                            self.hours = 0;
                            self.minutes = 10;
                            self.seconds = 0;
                        }
                        if ui.button("30 min").clicked() {
                            self.hours = 0;
                            self.minutes = 30;
                            self.seconds = 0;
                        }
                    });

                    ui.add_space(20.0);

                    if ui.button("Start Timer").clicked() {
                        self.set_duration();
                        if self.duration.as_secs() > 0 {
                            self.state = TimerState::Running;
                            self.elapsed = Duration::from_secs(0);
                        }
                    }
                }

                TimerState::Running | TimerState::Paused | TimerState::Completed => {
                    let text = match self.state {
                        TimerState::Completed => "Time's up!",
                        _ => "Remaining Time",
                    };

                    ui.label(RichText::new(text).font(FontId::proportional(18.0)));

                    let remaining = Self::format_time(self.remaining_time());

                    let timer_size = Vec2::splat(ui.available_width().min(240.0));
                    let (response, painter) = ui.allocate_painter(timer_size, egui::Sense::hover());
                    let center = response.rect.center();
                    let radius = response.rect.width() / 2.0 - 10.0;

                    let progress = self.progress();
                    let angle = std::f32::consts::TAU * progress - std::f32::consts::FRAC_PI_2;

                    painter.circle_stroke(center, radius, Stroke::new(5.0, Color32::DARK_GRAY));

                    let color = match self.state {
                        TimerState::Completed => Color32::from_rgb(235, 87, 87), // Red
                        TimerState::Paused => Color32::from_rgb(252, 186, 3),    // Amber
                        TimerState::Running => Color32::from_rgb(79, 134, 198),  // Blue
                        _ => Color32::GRAY,
                    };

                    if self.state != TimerState::Stopped && progress > 0.0 {
                        let segments = 100;
                        let mut last_point = egui::Pos2::new(
                            center.x + radius * (-std::f32::consts::FRAC_PI_2).cos(),
                            center.y + radius * (-std::f32::consts::FRAC_PI_2).sin(),
                        );

                        for i in 1..=((segments as f32) * progress).ceil() as usize {
                            let segment_angle = std::f32::consts::TAU
                                * (i as f32 / segments as f32)
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

                    painter.text(
                        center,
                        Align2::CENTER_CENTER,
                        remaining,
                        FontId::proportional(32.0),
                        if self.state == TimerState::Completed {
                            color
                        } else {
                            Color32::WHITE
                        },
                    );

                    ui.add_space(30.0);

                    ui.horizontal(|ui| {
                        match self.state {
                            TimerState::Running => {
                                if ui.button("Pause").clicked() {
                                    self.state = TimerState::Paused;
                                }
                            }
                            TimerState::Paused => {
                                if ui.button("Resume").clicked() {
                                    self.state = TimerState::Running;
                                }
                            }
                            TimerState::Completed => {}
                            TimerState::Stopped => todo!(),
                        }

                        if ui.button("Reset").clicked() {
                            self.state = TimerState::Stopped;
                            self.elapsed = Duration::from_secs(0);
                        }
                    });
                }
            }
        });
    }
}
