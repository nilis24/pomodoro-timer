use eframe::egui;

use crate::app::PomodoroApp;
use crate::business::pomodoro;
use crate::ui_helpers::centered_row;

pub fn show(app: &mut PomodoroApp, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Pomodoro per temps disponible");

        ui.add_space(18.0);

        centered_row(ui, 270.0, |ui| {
            ui.label("Tinc disponibles:");

            ui.add(
                egui::DragValue::new(&mut app.available_hours)
                    .range(0..=24)
                    .suffix(" h"),
            );

            ui.add(
                egui::DragValue::new(&mut app.available_minutes)
                    .range(0..=59)
                    .suffix(" min"),
            );
        });

        ui.add_space(18.0);

        if ui.button("Iniciar pomodoro").clicked() {
            println!("{}", pomodoro::start_with_available_time());
        }
    });
}
