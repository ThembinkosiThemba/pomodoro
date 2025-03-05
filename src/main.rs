mod app;
mod clock;
mod pomodoro;
mod sound;
mod stats;
mod task;
mod theme;
mod timer;

use app::ClockApp;
use egui::ViewportBuilder;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Rust Clock",
        options,
        Box::new(|cc| {
            // Set custom fonts if needed

            // Set default theme
            theme::set_theme(&cc.egui_ctx);

            Ok(Box::new(ClockApp::new(cc)))
        }),
    )
}
