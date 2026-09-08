use eframe::egui;

use crate::app::PomodoroApp;
use crate::business::config::{
    MAX_AVAILABLE_HOURS, MAX_AVAILABLE_MINUTES, MINUTE_STEP, MINUTES_INPUT_MAX, MINUTES_INPUT_MIN,
};
use crate::business::pomodoro::{self, PlanExecution};
use crate::screens::timer;
use crate::ui_helpers::centered_row;

const HOURS_INPUT_WIDTH: f32 = 86.0;
const MINUTES_INPUT_WIDTH: f32 = 86.0;

pub fn show(app: &mut PomodoroApp, ui: &mut egui::Ui) {
    if app.active_execution.is_some() {
        timer::show_execution(app, ui);
        return;
    }

    ui.vertical_centered(|ui| {
        ui.heading("Pomodoro per temps disponible");

        ui.add_space(18.0);

        let mut hours_input = app.available_hours as i32;
        let mut minutes_input = app.available_minutes as i32;

        centered_row(ui, 260.0, |ui| {
            ui.label("Tinc disponibles:");

            let hours_response = put_time_drag_value(
                ui,
                HOURS_INPUT_WIDTH,
                egui::DragValue::new(&mut hours_input)
                    .range(0..=MAX_AVAILABLE_HOURS as i32)
                    .speed(1)
                    .suffix(" h"),
            );

            let minutes_response = put_time_drag_value(
                ui,
                MINUTES_INPUT_WIDTH,
                egui::DragValue::new(&mut minutes_input)
                    .range(MINUTES_INPUT_MIN..=MINUTES_INPUT_MAX)
                    .speed(MINUTE_STEP)
                    .suffix(" min"),
            );

            if hours_response.changed() {
                set_available_time(app, hours_input * 60 + app.available_minutes as i32);
            }

            if minutes_response.changed() {
                set_available_time(
                    app,
                    app.available_hours as i32 * 60 + normalize_minutes_input(minutes_input),
                );
            }
        });

        let available_minutes = app.available_hours * 60 + app.available_minutes;
        let plan_without_extra = pomodoro::calculate_plan(
            available_minutes,
            app.config.work_minutes,
            app.config.short_break_minutes,
            app.config.long_break_minutes,
            app.config.long_break_every,
            app.config.min_extra_work_minutes,
            false,
        );
        let plan_with_extra = pomodoro::calculate_plan(
            available_minutes,
            app.config.work_minutes,
            app.config.short_break_minutes,
            app.config.long_break_minutes,
            app.config.long_break_every,
            app.config.min_extra_work_minutes,
            true,
        );
        let selected_plan = if app.use_remaining_for_extra_session && plan_with_extra.extra_session
        {
            &plan_with_extra
        } else {
            &plan_without_extra
        };

        ui.add_space(12.0);
        ui.label(format!("Faràs {} cicles de treball", selected_plan.cycles));

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
                app.config.work_minutes,
                app.config.short_break_minutes,
                app.config.long_break_minutes,
                app.config.long_break_every,
                app.config.min_extra_work_minutes,
                app.use_remaining_for_extra_session,
            );

            app.active_execution = Some(PlanExecution::start(plan));
        }
    });
}

fn max_available_minutes() -> i32 {
    (MAX_AVAILABLE_HOURS * 60 + MAX_AVAILABLE_MINUTES) as i32
}

fn put_time_drag_value(
    ui: &mut egui::Ui,
    width: f32,
    widget: egui::DragValue<'_>,
) -> egui::Response {
    let layout = egui::Layout::left_to_right(egui::Align::Center);

    ui.allocate_ui_with_layout(
        egui::vec2(width, ui.spacing().interact_size.y),
        layout,
        |ui| ui.add(widget),
    )
    .inner
}

fn set_available_time(app: &mut PomodoroApp, total_minutes: i32) {
    let total_minutes = total_minutes.clamp(0, max_available_minutes());
    app.available_hours = (total_minutes / 60) as u32;
    app.available_minutes = (total_minutes % 60) as u32;
}

fn normalize_minutes_input(minutes: i32) -> i32 {
    if minutes >= 0 {
        let remainder = minutes % MINUTE_STEP;
        if remainder == 0 {
            minutes
        } else {
            minutes + MINUTE_STEP - remainder
        }
    } else {
        minutes.div_euclid(MINUTE_STEP) * MINUTE_STEP
    }
}
