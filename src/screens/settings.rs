use eframe::egui;

use crate::app::{PomodoroApp, Tab};
use crate::business::config::{
    MAX_LONG_BREAK_EVERY, MAX_LONG_BREAK_MINUTES, MAX_SHORT_BREAK_MINUTES, MAX_WORK_MINUTES,
    MIN_BREAK_MINUTES, MIN_EXTRA_WORK_MINUTES, MIN_LONG_BREAK_EVERY, MIN_WORK_MINUTES,
};
use crate::ui_helpers::centered_row;

pub fn show(app: &mut PomodoroApp, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Configuració");

        ui.add_space(18.0);

        centered_row(ui, 145.0, |ui| {
            ui.label("Treball:");

            ui.add_sized(
                [66.0, ui.spacing().interact_size.y],
                egui::DragValue::new(&mut app.config.work_minutes)
                    .range(MIN_WORK_MINUTES..=MAX_WORK_MINUTES)
                    .suffix(" min"),
            );
        });

        app.config = app.config.clone().normalized();

        ui.add_space(8.0);

        centered_row(ui, 180.0, |ui| {
            ui.label("Descans curt:");

            ui.add_sized(
                [66.0, ui.spacing().interact_size.y],
                egui::DragValue::new(&mut app.config.short_break_minutes)
                    .range(MIN_BREAK_MINUTES..=MAX_SHORT_BREAK_MINUTES)
                    .suffix(" min"),
            );
        });

        ui.add_space(8.0);

        centered_row(ui, 188.0, |ui| {
            ui.label("Descans llarg:");

            ui.add_sized(
                [66.0, ui.spacing().interact_size.y],
                egui::DragValue::new(&mut app.config.long_break_minutes)
                    .range(MIN_BREAK_MINUTES..=MAX_LONG_BREAK_MINUTES)
                    .suffix(" min"),
            );
        });

        ui.add_space(8.0);

        centered_row(ui, 270.0, |ui| {
            ui.label("Descans llarg cada:");

            ui.add_sized(
                [84.0, ui.spacing().interact_size.y],
                egui::DragValue::new(&mut app.config.long_break_every)
                    .range(MIN_LONG_BREAK_EVERY..=MAX_LONG_BREAK_EVERY)
                    .suffix(" cicles"),
            );
        });

        ui.add_space(8.0);

        let max_extra_work_minutes = app.config.max_extra_work_minutes();

        centered_row(ui, 245.0, |ui| {
            ui.label("Mínim sessió extra:");

            ui.add_sized(
                [66.0, ui.spacing().interact_size.y],
                egui::DragValue::new(&mut app.config.min_extra_work_minutes)
                    .range(MIN_EXTRA_WORK_MINUTES..=max_extra_work_minutes)
                    .suffix(" min"),
            );
        });

        ui.add_space(18.0);

        if ui.button("Aplica").clicked() {
            app.config = app.config.clone().normalized();
            app.remaining_seconds = app.config.work_minutes * 60;
            if let Err(error) = app.config_repository.save(&app.config) {
                eprintln!("No s'ha pogut desar la configuracio: {error}");
            }
            app.tab = Tab::AvailableTime;
        }
    });
}
