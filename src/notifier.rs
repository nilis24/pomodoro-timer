use std::io::Cursor;
use std::thread;

use notify_rust::Notification;

use crate::business::pomodoro::{PhaseCompletion, PhaseKind, PlanPhase};

const NORMAL_NOTIFICATION_SOUND: &[u8] = include_bytes!("../assets/normal-notification.mp3");
const FINAL_NOTIFICATION_SOUND: &[u8] = include_bytes!("../assets/final-notification.mp3");

pub trait PhaseNotifier {
    fn notify_phase_completion(&self, completion: &PhaseCompletion);
}

#[derive(Debug, Default)]
pub struct DesktopPhaseNotifier;

impl PhaseNotifier for DesktopPhaseNotifier {
    fn notify_phase_completion(&self, completion: &PhaseCompletion) {
        match &completion.next_phase {
            Some(next_phase) => {
                play_sound(NORMAL_NOTIFICATION_SOUND);
                show_desktop_notification(
                    "Fase acabada",
                    &format!(
                        "{} acabada. Comença {}.",
                        phase_label(&completion.completed_phase),
                        phase_label(next_phase).to_lowercase()
                    ),
                );
            }
            None => {
                play_sound(FINAL_NOTIFICATION_SOUND);
                show_desktop_notification(
                    "Pla completat",
                    "Victoria! S'han acabat totes les fases del pomodoro.",
                );
            }
        }
    }
}

fn show_desktop_notification(summary: &str, body: &str) {
    if let Err(error) = Notification::new()
        .appname("Pomodoro")
        .summary(summary)
        .body(body)
        .icon("pomodoro-timer")
        .show()
    {
        eprintln!("No s'ha pogut mostrar la notificacio: {error}");
    }
}

fn play_sound(bytes: &'static [u8]) {
    thread::spawn(move || {
        if let Err(error) = play_sound_blocking(bytes) {
            eprintln!("No s'ha pogut reproduir la notificacio: {error}");
        }
    });
}

fn play_sound_blocking(
    bytes: &'static [u8],
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
    let player = rodio::Player::connect_new(stream_handle.mixer());
    let cursor = Cursor::new(bytes);

    player.append(rodio::Decoder::try_from(cursor)?);
    player.sleep_until_end();

    Ok(())
}

fn phase_label(phase: &PlanPhase) -> &'static str {
    match phase.kind {
        PhaseKind::Work => "Treball",
        PhaseKind::ShortBreak => "Descans curt",
        PhaseKind::LongBreak => "Descans llarg",
        PhaseKind::ExtraWork => "Treball extra",
    }
}
