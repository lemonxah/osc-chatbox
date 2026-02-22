use super::Module;
use crate::smalltext::to_superscript;
use std::fs;
use std::process::Command;
use sysinfo::System;

const TICKS_PER_SECTION: usize = 4; // ~1.5s per tick × 4 = ~6s per section

pub struct SystemDetailsModule {
    enabled: bool,
    parts: Vec<String>,
    index: usize,
    tick_count: usize,
}

fn clean_gpu_name(name: &str) -> String {
    let name = name
        .replace("NVIDIA", "")
        .replace("GeForce", "")
        .replace("Advanced Micro Devices, Inc.", "")
        .replace("Advanced Micro Devices", "")
        .replace("AMD", "")
        .replace("[", "")
        .replace("]", "");
    // Collapse multiple spaces
    name.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn detect_gpu() -> String {
    if let Ok(entries) = fs::read_dir("/proc/driver/nvidia/gpus") {
        for entry in entries.flatten() {
            if let Ok(info) = fs::read_to_string(entry.path().join("information")) {
                for line in info.lines() {
                    if line.starts_with("Model:") {
                        let raw = line.trim_start_matches("Model:").trim();
                        return clean_gpu_name(raw);
                    }
                }
            }
        }
    }

    if let Ok(output) = Command::new("lspci").output() {
        let text = String::from_utf8_lossy(&output.stdout);
        for line in text.lines() {
            if line.contains("VGA") || line.contains("3D controller") {
                if let Some(pos) = line.find(": ") {
                    return clean_gpu_name(&line[pos + 2..]);
                }
            }
        }
    }

    "Unknown".to_string()
}

fn detect_cpu_name(sys: &System) -> String {
    if let Some(cpu) = sys.cpus().first() {
        let brand = cpu.brand().trim().to_string();
        if !brand.is_empty() {
            // Remove core count suffix like "12-Core Processor" or "6-Core Processor"
            let lower = brand.to_lowercase();
            if let Some(pos) = lower.find("-core") {
                // Walk back to find the start of the digit(s) before "-core"
                let prefix = &brand[..pos];
                let trimmed = prefix
                    .trim_end_matches(|c: char| c.is_ascii_digit())
                    .trim_end();
                if !trimmed.is_empty() {
                    return trimmed.to_string();
                }
            }
            return brand;
        }
    }
    "Unknown".to_string()
}

impl SystemDetailsModule {
    pub fn new() -> Self {
        let sys = System::new_all();

        let os_name = System::name().unwrap_or_default();
        let os_version = System::os_version().unwrap_or_default();
        let cpu = detect_cpu_name(&sys);
        let gpu = detect_gpu();
        let raw_gb = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let total_ram = 2u64.pow((raw_gb.log2()).ceil() as u32);

        let os_version = os_version.replace("Rolling", "").trim().to_string();

        let parts = vec![
            to_superscript(&format!("{os_name} {os_version}").to_lowercase()),
            to_superscript(&cpu.to_lowercase()),
            to_superscript(&gpu.to_lowercase()),
            to_superscript(&format!("{total_ram}gb")),
        ];

        Self {
            enabled: false,
            parts,
            index: 0,
            tick_count: 0,
        }
    }
}

impl Module for SystemDetailsModule {
    fn name(&self) -> &str {
        "System Details"
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn tick(&mut self) -> Option<String> {
        if !self.enabled || self.parts.is_empty() {
            return None;
        }
        let part = self.parts[self.index].clone();
        self.tick_count += 1;
        if self.tick_count >= TICKS_PER_SECTION {
            self.tick_count = 0;
            self.index = (self.index + 1) % self.parts.len();
        }
        Some(part)
    }
}
