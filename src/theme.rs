// File: src/theme.rs
use eframe::egui::{self, Color32, Stroke, Visuals};

pub fn set_theme(ctx: &egui::Context) {
    let mut visuals = Visuals {
        dark_mode: true,
        override_text_color: Some(Color32::from_rgb(220, 220, 220)),
        panel_fill: Color32::from_rgb(30, 30, 35),
        window_fill: Color32::from_rgb(30, 30, 35),
        faint_bg_color: Color32::from_rgb(40, 40, 45),
        extreme_bg_color: Color32::from_rgb(20, 20, 25),
        code_bg_color: Color32::from_rgb(40, 40, 45),
        window_stroke: Stroke::new(1.0, Color32::from_rgb(60, 60, 65)),
        ..Default::default()
    };

    // Customize widget visuals
    visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(40, 40, 45);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(140, 140, 145));

    visuals.widgets.inactive.bg_fill = Color32::from_rgb(50, 50, 55);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(160, 160, 165));

    visuals.widgets.hovered.bg_fill = Color32::from_rgb(60, 60, 65);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::from_rgb(200, 200, 205));

    visuals.widgets.active.bg_fill = Color32::from_rgb(70, 70, 75);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::from_rgb(220, 220, 225));

    ctx.set_visuals(visuals);
}
