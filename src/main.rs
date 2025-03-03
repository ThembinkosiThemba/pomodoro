// File: src/main.rs
mod app;
mod clock;
mod pomodoro;
mod sound;
mod theme;
mod timer;
mod task;

use app::ClockApp;
use egui::ViewportBuilder;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Rust Clock",
        options,
        Box::new(|cc| {
            // Set custom fonts if needed
            // cc.egui_ctx.set_fonts(fonts);

            // Set default theme
            theme::set_theme(&cc.egui_ctx);

            Ok(Box::new(ClockApp::new(cc)))
        }),
    )
}
