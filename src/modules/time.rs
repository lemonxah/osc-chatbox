use super::Module;
use chrono::Local;

pub struct TimeModule {
    enabled: bool,
    format: String,
}

impl TimeModule {
    pub fn new() -> Self {
        Self {
            enabled: false,
            format: "%H:%M".to_string(),
        }
    }

    pub fn set_format(&mut self, format: String) {
        self.format = format;
    }
}

impl Module for TimeModule {
    fn name(&self) -> &str {
        "Time"
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn tick(&mut self) -> Option<String> {
        if !self.enabled {
            return None;
        }
        let now = Local::now();
        Some(now.format(&self.format).to_string())
    }
}
