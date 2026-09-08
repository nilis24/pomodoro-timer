use std::fs;
use std::path::PathBuf;

use chrono::{DateTime, Local};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::business::pomodoro::PlanExecution;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeLogEntry {
    pub started_at: DateTime<Local>,
    pub ended_at: DateTime<Local>,
    pub work_seconds: u32,
    pub break_seconds: u32,
    pub completed: bool,
}

impl TimeLogEntry {
    pub fn from_execution(execution: &PlanExecution, completed: bool) -> Self {
        Self {
            started_at: execution.started_at,
            ended_at: Local::now(),
            work_seconds: execution.worked_seconds(),
            break_seconds: execution.break_seconds(),
            completed,
        }
    }
}

#[derive(Debug, Clone)]
pub struct JsonTimeLogRepository {
    path: PathBuf,
}

impl Default for JsonTimeLogRepository {
    fn default() -> Self {
        Self {
            path: default_time_log_path(),
        }
    }
}

impl JsonTimeLogRepository {
    pub fn load(&self) -> Result<Vec<TimeLogEntry>, TimeLogError> {
        match fs::read_to_string(&self.path) {
            Ok(contents) => Ok(serde_json::from_str(&contents)?),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
            Err(error) => Err(error.into()),
        }
    }

    pub fn append(&self, entry: TimeLogEntry) -> Result<Vec<TimeLogEntry>, TimeLogError> {
        let mut entries = self.load()?;
        entries.push(entry);
        entries.sort_by(|a, b| b.started_at.cmp(&a.started_at));

        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&self.path, serde_json::to_string_pretty(&entries)?)?;

        Ok(entries)
    }
}

#[derive(Debug)]
pub enum TimeLogError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl std::fmt::Display for TimeLogError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "{error}"),
            Self::Json(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for TimeLogError {}

impl From<std::io::Error> for TimeLogError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for TimeLogError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

fn default_time_log_path() -> PathBuf {
    ProjectDirs::from("com", "Nilis", "Pomodoro")
        .map(|dirs| dirs.data_local_dir().join("time-log.json"))
        .unwrap_or_else(|| PathBuf::from("time-log.json"))
}
