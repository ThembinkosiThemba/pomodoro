use std::{fs, path::Path, time::Duration};

use crate::pomodoro::{Pomodoro, PomodoroState};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Task {
    pub name: String,
    pub duration: Duration,
    pub completed: bool,
    pub running: bool, // tracking running task
}

pub struct TaskList {
    tasks: Vec<Task>,
    finished_tasks: Vec<Task>,
    running_task_index: Option<usize>, // tracking the currently running task
}

impl TaskList {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            finished_tasks: Vec::new(),
            running_task_index: None,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, pomodoro: &mut Pomodoro) {
        ui.heading("Tasks");
        ui.add_space(10.0);

        let mut to_remove = None;
        for (i, task) in self.tasks.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.checkbox(&mut task.completed, "");
                ui.label(&task.name);
                ui.add_space(5.0);
                let mins = task.duration.as_secs() / 60;
                ui.label(format!("({} min)", mins));
                ui.label(if task.running { " (Running)" } else { "" }); // Indicate running task

                if ui.button("Start").clicked()
                    && !task.completed
                    && self.running_task_index.is_none()
                {
                    task.running = true;
                    self.running_task_index = Some(i);
                    pomodoro.work_duration = task.duration;
                    pomodoro.state = crate::pomodoro::PomodoroState::Work;
                    pomodoro.elapsed = Duration::from_secs(0);
                }
                if ui.button("Delete").clicked() {
                    to_remove = Some(i);
                }
            });
        }

        if let Some(index) = to_remove {
            self.tasks.remove(index);
        }

        if let Some(index) = self.running_task_index {
            if pomodoro.state != PomodoroState::Work && pomodoro.elapsed >= pomodoro.work_duration {
                if let Some(task) = self.tasks.get_mut(index) {
                    task.running = false;
                    task.completed = true;
                    self.finished_tasks.push(task.clone());
                    self.tasks.remove(index);
                }
                self.running_task_index = None;
                pomodoro.elapsed = Duration::from_secs(0); // Reset Pomodoro
            }
        }

        ui.add_space(10.0);
        ui.horizontal(|ui| {
            static mut NEW_TASK_NAME: String = String::new();
            static mut NEW_TASK_DURATION: u64 = 25;

            ui.text_edit_singleline(unsafe { &mut NEW_TASK_NAME });
            ui.add(
                egui::DragValue::new(unsafe { &mut NEW_TASK_DURATION })
                    .range(1..=120)
                    .suffix(" min"),
            );

            if ui.button("Add Task").clicked() {
                self.tasks.push(Task {
                    name: unsafe { NEW_TASK_NAME.clone() },
                    duration: Duration::from_secs(unsafe { NEW_TASK_DURATION } * 60),
                    completed: false,
                    running: false,
                });
                unsafe { NEW_TASK_NAME.clear() }
            }
        });

        ui.add_space(10.0);
        ui.heading("Finished Tasks");
        for task in &self.finished_tasks {
            ui.horizontal(|ui| {
                ui.label(&task.name);
                let mins = task.duration.as_secs() / 60;
                ui.label(format!("({} min)", mins));
            });
        }
    }

    pub fn save_to_file(&self) {
        if let Some(home_dir) = dirs::home_dir() {
            let backup_dir = home_dir.join(".rust_pomodoro_backup");
            fs::create_dir_all(&backup_dir).unwrap_or(());
            let tasks_json = serde_json::to_string(&self.tasks).unwrap_or_default();
            fs::write(backup_dir.join("tasks.json"), tasks_json).unwrap_or(());
        }
    }

    pub fn load_from_file() -> Self {
        if let Some(home_dir) = dirs::home_dir() {
            let file_path = home_dir.join(".rust_pomodoro_backup/tasks.json");
            if Path::new(&file_path).exists() {
                if let Ok(contents) = fs::read_to_string(file_path) {
                    if let Ok(tasks) = serde_json::from_str(&contents) {
                        return Self {
                            tasks,
                            finished_tasks: Vec::new(),
                            running_task_index: None,
                        };
                    }
                }
            }
        }
        Self::new()
    }
}
