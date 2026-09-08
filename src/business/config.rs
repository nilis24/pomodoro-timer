use std::fs;
use std::path::PathBuf;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

pub const DEFAULT_AVAILABLE_HOURS: u32 = 2;
pub const DEFAULT_AVAILABLE_MINUTES: u32 = 0;
pub const DEFAULT_CYCLE_COUNT: u32 = 4;
pub const DEFAULT_WORK_MINUTES: u32 = 25;
pub const DEFAULT_SHORT_BREAK_MINUTES: u32 = 5;
pub const DEFAULT_LONG_BREAK_MINUTES: u32 = 15;
pub const DEFAULT_LONG_BREAK_EVERY: u32 = 4;
pub const DEFAULT_MIN_EXTRA_WORK_MINUTES: u32 = 10;

pub const MIN_WORK_MINUTES: u32 = 11;
pub const MAX_WORK_MINUTES: u32 = 180;
pub const MIN_BREAK_MINUTES: u32 = 1;
pub const MAX_SHORT_BREAK_MINUTES: u32 = 60;
pub const MAX_LONG_BREAK_MINUTES: u32 = 120;
pub const MIN_LONG_BREAK_EVERY: u32 = 1;
pub const MAX_LONG_BREAK_EVERY: u32 = 24;
pub const MIN_EXTRA_WORK_MINUTES: u32 = 10;
pub const MIN_CYCLE_COUNT: u32 = 1;
pub const MAX_CYCLE_COUNT: u32 = 24;
pub const MAX_AVAILABLE_HOURS: u32 = 24;
pub const MAX_AVAILABLE_MINUTES: u32 = 59;
pub const MINUTE_STEP: i32 = 5;
pub const MINUTES_INPUT_MIN: i32 = -MINUTE_STEP;
pub const MINUTES_INPUT_MAX: i32 = MAX_AVAILABLE_MINUTES as i32 + MINUTE_STEP;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub work_minutes: u32,
    pub short_break_minutes: u32,
    pub long_break_minutes: u32,
    pub long_break_every: u32,
    pub min_extra_work_minutes: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            work_minutes: DEFAULT_WORK_MINUTES,
            short_break_minutes: DEFAULT_SHORT_BREAK_MINUTES,
            long_break_minutes: DEFAULT_LONG_BREAK_MINUTES,
            long_break_every: DEFAULT_LONG_BREAK_EVERY,
            min_extra_work_minutes: DEFAULT_MIN_EXTRA_WORK_MINUTES,
        }
    }
}

impl AppConfig {
    pub fn normalized(mut self) -> Self {
        self.work_minutes = self.work_minutes.clamp(MIN_WORK_MINUTES, MAX_WORK_MINUTES);
        self.short_break_minutes = self
            .short_break_minutes
            .clamp(MIN_BREAK_MINUTES, MAX_SHORT_BREAK_MINUTES);
        self.long_break_minutes = self
            .long_break_minutes
            .clamp(MIN_BREAK_MINUTES, MAX_LONG_BREAK_MINUTES);
        self.long_break_every = self
            .long_break_every
            .clamp(MIN_LONG_BREAK_EVERY, MAX_LONG_BREAK_EVERY);
        self.min_extra_work_minutes = self
            .min_extra_work_minutes
            .clamp(MIN_EXTRA_WORK_MINUTES, self.max_extra_work_minutes());
        self
    }

    pub fn max_extra_work_minutes(&self) -> u32 {
        self.work_minutes
            .saturating_sub(1)
            .max(MIN_EXTRA_WORK_MINUTES)
    }
}

#[derive(Debug, Clone)]
pub struct TomlConfigRepository {
    path: PathBuf,
}

impl Default for TomlConfigRepository {
    fn default() -> Self {
        Self {
            path: default_config_path(),
        }
    }
}

impl TomlConfigRepository {
    pub fn load(&self) -> Result<AppConfig, ConfigError> {
        match fs::read_to_string(&self.path) {
            Ok(contents) => Ok(toml::from_str::<AppConfig>(&contents)?.normalized()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(AppConfig::default()),
            Err(error) => Err(error.into()),
        }
    }

    pub fn save(&self, config: &AppConfig) -> Result<(), ConfigError> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(
            &self.path,
            toml::to_string_pretty(&config.clone().normalized())?,
        )?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Decode(toml::de::Error),
    Encode(toml::ser::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "{error}"),
            Self::Decode(error) => write!(formatter, "{error}"),
            Self::Encode(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(error: toml::de::Error) -> Self {
        Self::Decode(error)
    }
}

impl From<toml::ser::Error> for ConfigError {
    fn from(error: toml::ser::Error) -> Self {
        Self::Encode(error)
    }
}

fn default_config_path() -> PathBuf {
    ProjectDirs::from("com", "Nilis", "pomodoro-timer")
        .map(|dirs| dirs.config_dir().join("config.toml"))
        .unwrap_or_else(|| PathBuf::from("config.toml"))
}
