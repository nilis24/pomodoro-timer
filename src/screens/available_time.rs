use eframe::egui;

use crate::app::PomodoroApp;
use crate::business::pomodoro::{self, ExecutionStatus, PhaseKind, PlanExecution};
use crate::ui_helpers::centered_row;

pub fn show(app: &mut PomodoroApp, ui: &mut egui::Ui) {
    if app.active_execution.is_some() {
        show_execution(app, ui);
        return;
    }

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
            app.long_break_every,
            app.min_extra_work_minutes,
            false,
        );
        let plan_with_extra = pomodoro::calculate_plan(
            available_minutes,
            app.work_minutes,
            app.short_break_minutes,
            app.long_break_minutes,
            app.long_break_every,
            app.min_extra_work_minutes,
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
                app.long_break_every,
                app.min_extra_work_minutes,
                app.use_remaining_for_extra_session,
            );

            app.active_execution = Some(PlanExecution::start(plan));
        }
    });
}

fn show_execution(app: &mut PomodoroApp, ui: &mut egui::Ui) {
    let mut stop_execution = false;

    if let Some(execution) = &mut app.active_execution {
        execution.tick();

        if execution.status == ExecutionStatus::Running {
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(250));
        }

        ui.vertical_centered(|ui| {
            ui.heading("Pomodoro en curs");
            ui.add_space(18.0);

            let total_phases = execution.plan.phases.len();
            let current_phase_number = if execution.status == ExecutionStatus::Finished {
                total_phases
            } else {
                execution.phase_index + 1
            };
            ui.label(format!("Fase {}/{}", current_phase_number, total_phases));

            if let Some(phase) = execution.current_phase() {
                ui.label(phase_label(&phase.kind));
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(format_time(execution.remaining_seconds))
                        .monospace()
                        .size(44.0),
                );
            } else {
                ui.label("Sessió finalitzada");
                ui.add_space(8.0);
                ui.label(egui::RichText::new("00:00").monospace().size(44.0));
            }

            ui.add_space(18.0);

            centered_row(ui, 292.0, |ui| {
                match execution.status {
                    ExecutionStatus::Running => {
                        if ui
                            .add_sized([92.0, 32.0], egui::Button::new("Pausa"))
                            .clicked()
                        {
                            execution.pause();
                        }
                    }
                    ExecutionStatus::Paused => {
                        if ui
                            .add_sized([92.0, 32.0], egui::Button::new("Play"))
                            .clicked()
                        {
                            execution.play();
                        }
                    }
                    ExecutionStatus::Finished => {
                        ui.add_enabled(
                            false,
                            egui::Button::new("Play").min_size(egui::vec2(92.0, 32.0)),
                        );
                    }
                }

                if ui
                    .add_sized([92.0, 32.0], egui::Button::new("Restablir"))
                    .clicked()
                {
                    execution.reset();
                }

                if ui
                    .add_sized([92.0, 32.0], egui::Button::new("Aturar"))
                    .clicked()
                {
                    stop_execution = true;
                }
            });
        });
    }

    if stop_execution {
        app.active_execution = None;
    }
}

fn phase_label(kind: &PhaseKind) -> &'static str {
    match kind {
        PhaseKind::Work => "Treball",
        PhaseKind::ShortBreak => "Descans curt",
        PhaseKind::LongBreak => "Descans llarg",
        PhaseKind::ExtraWork => "Treball extra",
    }
}

fn format_time(seconds: u32) -> String {
    format!("{:02}:{:02}", seconds / 60, seconds % 60)
}
