use eframe::egui;

use crate::app::PomodoroApp;
use crate::business::pomodoro::{self, PlanExecution};
use crate::screens::timer;
use crate::ui_helpers::centered_row;

pub fn show(app: &mut PomodoroApp, ui: &mut egui::Ui) {
    if app.active_execution.is_some() {
        timer::show_execution(app, ui);
        return;
    }

    ui.vertical_centered(|ui| {
        ui.heading("Pomodoro per cicles");

        ui.add_space(18.0);

        centered_row(ui, 165.0, |ui| {
            ui.label("Vull fer:");

            ui.add_sized(
                [84.0, ui.spacing().interact_size.y],
                egui::DragValue::new(&mut app.cycle_count)
                    .range(1..=24)
                    .suffix(" cicles"),
            );
        });

        let plan = pomodoro::calculate_cycle_plan(
            app.cycle_count,
            app.work_minutes,
            app.short_break_minutes,
            app.long_break_minutes,
            app.long_break_every,
        );

        ui.add_space(12.0);
        ui.label(format!(
            "Total: {} de treball + {} de descans",
            format_duration(plan.work_time),
            format_duration(plan.break_time)
        ));
        ui.label(format!(
            "Durada prevista: {}",
            format_duration(plan.used_time)
        ));

        ui.add_space(18.0);

        if ui.button("Iniciar pomodoro").clicked() {
            app.active_execution = Some(PlanExecution::start(plan));
        }
    });
}

fn format_duration(minutes: u32) -> String {
    let hours = minutes / 60;
    let minutes = minutes % 60;

    match (hours, minutes) {
        (0, minutes) => format!("{minutes} min"),
        (hours, 0) => format!("{hours} h"),
        (hours, minutes) => format!("{hours} h {minutes} min"),
    }
}
