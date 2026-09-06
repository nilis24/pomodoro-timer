use eframe::egui;

use crate::app::PomodoroApp;
use crate::ui_helpers::centered_row;

pub fn show(app: &mut PomodoroApp, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Pomodoro per cicles");

        ui.add_space(18.0);

        centered_row(ui, 175.0, |ui| {
            ui.label("Vull fer:");

            ui.add(
                egui::DragValue::new(&mut app.cycle_count)
                    .range(1..=24)
                    .suffix(" cicles"),
            );
        });

        ui.add_space(18.0);

        if ui.button("Iniciar pomodoro").clicked() {
            println!("Inicia amb nombre de cicles: {}", app.cycle_count);
        }
    });
}
