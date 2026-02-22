use super::Module;
use crate::smalltext::to_superscript;
use sysinfo::System;

pub struct StatsModule {
    enabled: bool,
    system: System,
    show_cpu: bool,
    show_ram: bool,
}

impl StatsModule {
    pub fn new() -> Self {
        Self {
            enabled: false,
            system: System::new_all(),
            show_cpu: true,
            show_ram: true,
        }
    }

    pub fn set_show_cpu(&mut self, show: bool) {
        self.show_cpu = show;
    }

    pub fn set_show_ram(&mut self, show: bool) {
        self.show_ram = show;
    }
}

impl Module for StatsModule {
    fn name(&self) -> &str {
        "Stats"
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

        self.system.refresh_cpu_usage();
        self.system.refresh_memory();

        let mut parts = Vec::new();

        if self.show_cpu {
            let cpu_usage = self.system.global_cpu_usage();
            parts.push(format!(
                "CPU {}",
                to_superscript(&format!("{cpu_usage:.0}%"))
            ));
        }

        if self.show_ram {
            let used = self.system.used_memory() / 1024 / 1024;
            let total = self.system.total_memory() / 1024 / 1024;
            parts.push(format!(
                "RAM {}",
                to_superscript(&format!("{used}/{total}mb"))
            ));
        }

        if parts.is_empty() {
            return None;
        }

        Some(parts.join(" | "))
    }
}
