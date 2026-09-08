use eframe::egui;

use crate::app::PomodoroApp;
use crate::business::time_log::TimeLogEntry;

pub fn show(app: &PomodoroApp, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Registre de temps");
    });

    ui.add_space(18.0);

    let column_spacing = 16.0;
    let column_width = (ui.available_width() - column_spacing * 3.0) / 4.0;

    egui::Grid::new("time_log_table")
        .num_columns(4)
        .striped(true)
        .spacing([column_spacing, 10.0])
        .min_col_width(column_width)
        .show(ui, |ui| {
            ui.add_sized(
                [column_width, 20.0],
                egui::Label::new(egui::RichText::new("Dia i hora").strong()),
            );
            ui.add_sized(
                [column_width, 20.0],
                egui::Label::new(egui::RichText::new("Temps treballat").strong()),
            );
            ui.add_sized(
                [column_width, 20.0],
                egui::Label::new(egui::RichText::new("Temps de descans").strong()),
            );
            ui.add_sized(
                [column_width, 20.0],
                egui::Label::new(egui::RichText::new("Estat").strong()),
            );
            ui.end_row();

            for entry in &app.time_log_entries {
                show_entry_row(ui, entry, column_width);
            }
        });
}

fn show_entry_row(ui: &mut egui::Ui, entry: &TimeLogEntry, column_width: f32) {
    ui.add_sized(
        [column_width, 20.0],
        egui::Label::new(entry.started_at.format("%d/%m/%Y %H:%M").to_string()),
    );
    ui.add_sized(
        [column_width, 20.0],
        egui::Label::new(format_duration(entry.work_seconds)),
    );
    ui.add_sized(
        [column_width, 20.0],
        egui::Label::new(format_duration(entry.break_seconds)),
    );
    ui.add_sized(
        [column_width, 20.0],
        egui::Label::new(if entry.completed {
            "Completat"
        } else {
            "Aturat"
        }),
    );
    ui.end_row();
}

fn format_duration(seconds: u32) -> String {
    let total_minutes = seconds / 60;
    let hours = total_minutes / 60;
    let minutes = total_minutes % 60;

    match (hours, minutes, seconds) {
        (0, 0, 0) => "0 min".to_owned(),
        (0, 0, _) => "<1 min".to_owned(),
        (0, minutes, _) => format!("{minutes} min"),
        (hours, 0, _) => format!("{hours} h"),
        (hours, minutes, _) => format!("{hours} h {minutes} min"),
    }
}
