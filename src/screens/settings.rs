use eframe::egui;

use crate::app::{PomodoroApp, Tab};
use crate::ui_helpers::centered_row;

pub fn show(app: &mut PomodoroApp, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Configuració");

        ui.add_space(18.0);

        centered_row(ui, 145.0, |ui| {
            ui.label("Treball:");

            ui.add_sized(
                [66.0, ui.spacing().interact_size.y],
                egui::DragValue::new(&mut app.work_minutes)
                    .range(11..=180)
                    .suffix(" min"),
            );
        });

        app.min_extra_work_minutes = app
            .min_extra_work_minutes
            .clamp(10, app.work_minutes.saturating_sub(1));

        ui.add_space(8.0);

        centered_row(ui, 180.0, |ui| {
            ui.label("Descans curt:");

            ui.add_sized(
                [66.0, ui.spacing().interact_size.y],
                egui::DragValue::new(&mut app.short_break_minutes)
                    .range(1..=60)
                    .suffix(" min"),
            );
        });

        ui.add_space(8.0);

        centered_row(ui, 188.0, |ui| {
            ui.label("Descans llarg:");

            ui.add_sized(
                [66.0, ui.spacing().interact_size.y],
                egui::DragValue::new(&mut app.long_break_minutes)
                    .range(1..=120)
                    .suffix(" min"),
            );
        });

        ui.add_space(8.0);

        centered_row(ui, 270.0, |ui| {
            ui.label("Descans llarg cada:");

            ui.add_sized(
                [84.0, ui.spacing().interact_size.y],
                egui::DragValue::new(&mut app.long_break_every)
                    .range(1..=24)
                    .suffix(" cicles"),
            );
        });

        ui.add_space(8.0);

        centered_row(ui, 245.0, |ui| {
            ui.label("Mínim sessió extra:");

            ui.add_sized(
                [66.0, ui.spacing().interact_size.y],
                egui::DragValue::new(&mut app.min_extra_work_minutes)
                    .range(10..=app.work_minutes - 1)
                    .suffix(" min"),
            );
        });

        ui.add_space(18.0);

        if ui.button("Aplica").clicked() {
            app.remaining_seconds = app.work_minutes * 60;
            app.tab = Tab::AvailableTime;
        }
    });
}
