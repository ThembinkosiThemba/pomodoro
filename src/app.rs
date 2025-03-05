use crate::pomodoro::{Pomodoro, PomodoroState};
use crate::stats::Stats;
use crate::timer::Timer;
use crate::{clock::Clock, task::TaskList};
use eframe::egui;
use egui::{Align, Layout, RichText, Ui};
use std::time::{Duration, Instant};

#[derive(PartialEq)]
pub enum Tab {
    Pomodoro,
    Clock,
    Timer,
}

pub struct ClockApp {
    clock: Clock,
    pomodoro: Pomodoro,
    timer: Timer,
    current_tab: Tab,
    last_update: Instant,
    task_list: TaskList,
    stats: Stats,
}

impl ClockApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            clock: Clock::new(),
            pomodoro: Pomodoro::new(),
            timer: Timer::new(),
            current_tab: Tab::Pomodoro,
            last_update: Instant::now(),
            task_list: TaskList::load_from_file(),
            stats: Stats::load(),
        }
    }

    fn render_tab_bar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.current_tab, Tab::Pomodoro, "Pomodoro");
            ui.selectable_value(&mut self.current_tab, Tab::Clock, "Clock");
            ui.selectable_value(&mut self.current_tab, Tab::Timer, "Timer");
        });
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);
    }

    fn render_footer(&mut self, ui: &mut Ui) {
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(5.0);

        ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
            ui.add_space(5.0);
            ui.label(RichText::new("Pomodoro By Bane v0.1.0").small().weak());
        });
    }

    fn update_timers(&mut self, ctx: &egui::Context) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update);
        self.last_update = now;

        self.pomodoro.update(elapsed, ctx);
        self.timer.update(elapsed, ctx);

        if self.pomodoro.state == PomodoroState::Work && elapsed > Duration::from_secs(0) {
            self.stats.add_work_time(elapsed);
        }

        if self.pomodoro.metrics.completed_pomodoros > 0 {
            self.stats.add_pomodoro();
            self.stats.save(); // Save stats after update
        }

        // Request a repaint for the next frame to keep the UI updating
        ctx.request_repaint();
        self.task_list.save_to_file();
    }
}

impl eframe::App for ClockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.clock.update();
        self.update_timers(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_tab_bar(ui);

            match self.current_tab {
                Tab::Pomodoro => {
                    ui.horizontal(|ui| {
                        ui.add_space(40.0);
                        ui.vertical(|ui| {
                            ui.set_width(ui.available_width() * 0.5);
                            self.pomodoro.ui(ui);
                        });

                        ui.vertical(|ui| {
                            ui.set_width(ui.available_width());
                            ui.add_space(20.0);
                            self.task_list.ui(ui, &mut self.pomodoro);
                        });
                    });
                }
                Tab::Clock => self.clock.ui(ui),
                Tab::Timer => self.timer.ui(ui),
            }

            self.render_footer(ui);
        });
    }
}
