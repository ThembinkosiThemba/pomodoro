use chrono::{DateTime, Local, Timelike};
use egui::{Color32, FontId, Pos2, RichText, Stroke, Ui, Vec2};
use std::f32::consts::PI;

pub struct Clock {
    current_time: DateTime<Local>,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            current_time: Local::now(),
        }
    }

    pub fn update(&mut self) {
        self.current_time = Local::now();
    }

    pub fn ui(&self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            // Digital clock display
            ui.add_space(20.0);
            let time_str = self.current_time.format("%H:%M:%S").to_string();
            ui.label(RichText::new(time_str).font(FontId::proportional(40.0)));

            let date_str = self.current_time.format("%A, %B %d, %Y").to_string();
            ui.label(RichText::new(date_str).font(FontId::proportional(16.0)));

            ui.add_space(30.0);

            // Analog clock
            let clock_size = Vec2::splat(ui.available_width().min(240.0));
            let (response, painter) = ui.allocate_painter(clock_size, egui::Sense::hover());
            let center = response.rect.center();
            let radius = response.rect.width() / 2.0 - 10.0;

            // Draw clock face
            painter.circle_stroke(center, radius, Stroke::new(2.0, Color32::GRAY));

            // Draw hour markers
            for i in 0..12 {
                let angle = i as f32 * PI / 6.0;
                let cos_a = angle.cos();
                let sin_a = angle.sin();

                let start_pos = Pos2::new(
                    center.x + (radius - 10.0) * cos_a,
                    center.y + (radius - 10.0) * sin_a,
                );
                let end_pos = Pos2::new(center.x + radius * cos_a, center.y + radius * sin_a);

                let thickness = if i % 3 == 0 { 2.0 } else { 1.0 };
                painter.line_segment([start_pos, end_pos], Stroke::new(thickness, Color32::GRAY));
            }

            // Get current time components for clock hands
            let hours =
                (self.current_time.hour() % 12) as f32 + self.current_time.minute() as f32 / 60.0;
            let minutes =
                self.current_time.minute() as f32 + self.current_time.second() as f32 / 60.0;
            let seconds = self.current_time.second() as f32;

            // Draw hour hand
            let hour_angle = hours * PI / 6.0 - PI / 2.0;
            let hour_hand_length = radius * 0.5;
            let hour_pos = Pos2::new(
                center.x + hour_hand_length * hour_angle.cos(),
                center.y + hour_hand_length * hour_angle.sin(),
            );
            painter.line_segment([center, hour_pos], Stroke::new(3.0, Color32::WHITE));

            // Draw minute hand
            let minute_angle = minutes * PI / 30.0 - PI / 2.0;
            let minute_hand_length = radius * 0.7;
            let minute_pos = Pos2::new(
                center.x + minute_hand_length * minute_angle.cos(),
                center.y + minute_hand_length * minute_angle.sin(),
            );
            painter.line_segment([center, minute_pos], Stroke::new(2.0, Color32::WHITE));

            // Draw second hand
            let second_angle = seconds * PI / 30.0 - PI / 2.0;
            let second_hand_length = radius * 0.8;
            let second_pos = Pos2::new(
                center.x + second_hand_length * second_angle.cos(),
                center.y + second_hand_length * second_angle.sin(),
            );
            painter.line_segment(
                [center, second_pos],
                Stroke::new(1.0, Color32::from_rgb(255, 100, 100)),
            );

            // Draw center dot
            painter.circle_filled(center, 4.0, Color32::WHITE);
        });
    }
}
