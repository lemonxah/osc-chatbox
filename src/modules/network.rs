use super::Module;
use crate::smalltext::to_superscript;
use sysinfo::Networks;

pub struct NetworkModule {
    enabled: bool,
    networks: Networks,
    prev_rx: u64,
    prev_tx: u64,
}

impl NetworkModule {
    pub fn new() -> Self {
        Self {
            enabled: false,
            networks: Networks::new_with_refreshed_list(),
            prev_rx: 0,
            prev_tx: 0,
        }
    }
}

fn format_rate(bytes: u64) -> String {
    if bytes > 1_048_576 {
        format!("{:.1}mb/s", bytes as f64 / 1_048_576.0)
    } else if bytes > 1024 {
        format!("{:.0}kb/s", bytes as f64 / 1024.0)
    } else {
        format!("{bytes}b/s")
    }
}

impl Module for NetworkModule {
    fn name(&self) -> &str {
        "Network"
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

        self.networks.refresh(true);

        let mut total_rx: u64 = 0;
        let mut total_tx: u64 = 0;

        for (_name, data) in &self.networks {
            total_rx += data.total_received();
            total_tx += data.total_transmitted();
        }

        let rx_rate = total_rx.saturating_sub(self.prev_rx);
        let tx_rate = total_tx.saturating_sub(self.prev_tx);

        self.prev_rx = total_rx;
        self.prev_tx = total_tx;

        Some(format!(
            "NET {} {}",
            to_superscript(&format!("down:{}", format_rate(rx_rate))),
            to_superscript(&format!("up:{}", format_rate(tx_rate)))
        ))
    }
}
