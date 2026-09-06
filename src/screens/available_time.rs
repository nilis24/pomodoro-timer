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

        let available_minutes = app.available_hours * 60 + app.available_minutes;
        let plan_without_extra = pomodoro::calculate_plan(
            available_minutes,
            app.work_minutes,
            app.short_break_minutes,
            app.long_break_minutes,
            4,
            false,
        );
        let plan_with_extra = pomodoro::calculate_plan(
            available_minutes,
            app.work_minutes,
            app.short_break_minutes,
            app.long_break_minutes,
            4,
            true,
        );

        if plan_without_extra.remaining_time > 0 {
            ui.add_space(12.0);
            ui.label(format!(
                "Sobraran {} min sense sessió extra",
                plan_without_extra.remaining_time
            ));
            if plan_with_extra.extra_session {
                ui.checkbox(
                    &mut app.use_remaining_for_extra_session,
                    "Afegir el descans que toca i fer el temps restant de treball",
                );

                if app.use_remaining_for_extra_session {
                    let extra_break = plan_with_extra.break_time - plan_without_extra.break_time;

                    ui.label(format!(
                        "Extra final: {} min de descans + {} min de treball",
                        extra_break, plan_with_extra.extra_session_time
                    ));
                }
            } else {
                app.use_remaining_for_extra_session = false;
            }
        } else {
            app.use_remaining_for_extra_session = false;
        }

        ui.add_space(18.0);

        if ui.button("Iniciar pomodoro").clicked() {
            let plan = pomodoro::calculate_plan(
                available_minutes,
                app.work_minutes,
                app.short_break_minutes,
                app.long_break_minutes,
                4,
                app.use_remaining_for_extra_session,
            );

            println!("{plan:#?}");
        }
    });
}
