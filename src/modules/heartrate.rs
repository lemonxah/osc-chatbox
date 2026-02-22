use super::Module;
use std::sync::{Arc, Mutex};

pub struct HeartrateModule {
    enabled: bool,
    bpm: Arc<Mutex<Option<u32>>>,
    token: String,
}

impl HeartrateModule {
    pub fn new() -> Self {
        Self {
            enabled: false,
            bpm: Arc::new(Mutex::new(None)),
            token: String::new(),
        }
    }

    pub fn set_token(&mut self, token: String) {
        self.token = token;
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn bpm_handle(&self) -> Arc<Mutex<Option<u32>>> {
        self.bpm.clone()
    }

    pub fn set_bpm(&self, value: Option<u32>) {
        if let Ok(mut bpm) = self.bpm.lock() {
            *bpm = value;
        }
    }
}

impl Module for HeartrateModule {
    fn name(&self) -> &str {
        "Heart Rate"
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

        let bpm = self.bpm.lock().ok()?.as_ref().copied();
        bpm.map(|v| format!("{v} BPM"))
    }
}
