use super::Module;
use std::time::{Duration, Instant};

pub struct AfkModule {
    enabled: bool,
    timeout: Duration,
    last_activity: Instant,
    afk_text: String,
    is_afk: bool,
}

impl AfkModule {
    pub fn new() -> Self {
        Self {
            enabled: false,
            timeout: Duration::from_secs(300),
            last_activity: Instant::now(),
            afk_text: "AFK".to_string(),
            is_afk: false,
        }
    }

    pub fn set_timeout_secs(&mut self, secs: u64) {
        self.timeout = Duration::from_secs(secs);
    }

    pub fn set_afk_text(&mut self, text: String) {
        self.afk_text = text;
    }

    pub fn poke(&mut self) {
        self.last_activity = Instant::now();
        self.is_afk = false;
    }

    pub fn is_afk(&self) -> bool {
        self.is_afk
    }
}

impl Module for AfkModule {
    fn name(&self) -> &str {
        "AFK"
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

        if self.last_activity.elapsed() >= self.timeout {
            self.is_afk = true;
            Some(self.afk_text.clone())
        } else {
            self.is_afk = false;
            None
        }
    }
}
