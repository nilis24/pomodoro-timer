use eframe::egui::{self, FontId, TextStyle};

use crate::business::pomodoro::PlanExecution;
use crate::business::time_log::{JsonTimeLogRepository, TimeLogEntry};
use crate::notifier::{DesktopPhaseNotifier, PhaseNotifier};
use crate::screens;
use crate::ui_helpers::centered_row;

#[derive(PartialEq)]
pub enum Tab {
    AvailableTime,
    Cycles,
    TimeLog,
    Settings,
}

pub struct PomodoroApp {
    pub tab: Tab,
    pub remaining_seconds: u32,
    pub available_hours: u32,
    pub available_minutes: u32,
    pub cycle_count: u32,
    pub work_minutes: u32,
    pub short_break_minutes: u32,
    pub long_break_minutes: u32,
    pub long_break_every: u32,
    pub min_extra_work_minutes: u32,
    pub use_remaining_for_extra_session: bool,
    pub active_execution: Option<PlanExecution>,
    pub phase_notifier: Box<dyn PhaseNotifier>,
    pub time_log_repository: JsonTimeLogRepository,
    pub time_log_entries: Vec<TimeLogEntry>,
}

impl Default for PomodoroApp {
    fn default() -> Self {
        let time_log_repository = JsonTimeLogRepository::default();
        let time_log_entries = time_log_repository.load().unwrap_or_else(|error| {
            eprintln!("No s'ha pogut carregar el registre de temps: {error}");
            Vec::new()
        });

        Self {
            tab: Tab::AvailableTime,
            remaining_seconds: 25 * 60,
            available_hours: 2,
            available_minutes: 0,
            cycle_count: 4,
            work_minutes: 25,
            short_break_minutes: 5,
            long_break_minutes: 15,
            long_break_every: 4,
            min_extra_work_minutes: 10,
            use_remaining_for_extra_session: false,
            active_execution: None,
            phase_notifier: Box::<DesktopPhaseNotifier>::default(),
            time_log_repository,
            time_log_entries,
        }
    }
}

impl eframe::App for PomodoroApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        configure_text_styles(ui);

        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(8.0);

                centered_row(ui, 515.0, |ui| {
                    ui.selectable_value(&mut self.tab, Tab::AvailableTime, "Temps disponible");
                    ui.selectable_value(&mut self.tab, Tab::Cycles, "Cicles");
                    ui.selectable_value(&mut self.tab, Tab::TimeLog, "Registre de temps");
                    ui.selectable_value(&mut self.tab, Tab::Settings, "Configuració");
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                match self.tab {
                    Tab::AvailableTime => screens::available_time::show(self, ui),
                    Tab::Cycles => screens::cycles::show(self, ui),
                    Tab::TimeLog => screens::time_log::show(self, ui),
                    Tab::Settings => screens::settings::show(self, ui),
                }
            });
        });
    }
}

fn configure_text_styles(ui: &mut egui::Ui) {
    ui.style_mut().text_styles = [
        (TextStyle::Heading, FontId::proportional(26.0)),
        (TextStyle::Body, FontId::proportional(18.0)),
        (TextStyle::Button, FontId::proportional(18.0)),
        (TextStyle::Monospace, FontId::monospace(17.0)),
        (TextStyle::Small, FontId::proportional(15.0)),
    ]
    .into();
}
