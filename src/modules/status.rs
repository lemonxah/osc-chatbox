use super::Module;

const TICKS_PER_LINE: usize = 4; // ~1.5s per tick × 4 = ~6s per line

pub struct StatusModule {
    enabled: bool,
    lines: [String; 6],
    index: usize,
    tick_count: usize,
}

impl StatusModule {
    pub fn new() -> Self {
        Self {
            enabled: false,
            lines: Default::default(),
            index: 0,
            tick_count: 0,
        }
    }

    pub fn set_line(&mut self, index: usize, text: String) {
        if index < 6 {
            self.lines[index] = text;
        }
    }
}

impl Module for StatusModule {
    fn name(&self) -> &str {
        "Status"
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

        // Collect non-empty lines
        let active: Vec<&String> = self.lines.iter().filter(|l| !l.is_empty()).collect();
        if active.is_empty() {
            return None;
        }

        // Clamp index in case lines were removed
        if self.index >= active.len() {
            self.index = 0;
            self.tick_count = 0;
        }

        let line = active[self.index].clone();
        self.tick_count += 1;
        if self.tick_count >= TICKS_PER_LINE {
            self.tick_count = 0;
            self.index = (self.index + 1) % active.len();
        }
        Some(line)
    }
}
