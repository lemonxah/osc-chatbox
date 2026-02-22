use super::Module;
use std::process::Command;

const PREFERRED_PLAYERS: &[&str] = &["spotify", "spotifyd"];
const BAR_LENGTH: usize = 10;

pub struct MediaModule {
    enabled: bool,
}

struct MediaInfo {
    artist: String,
    title: String,
    status: String,
    position_us: u64,
    length_us: u64,
}

impl MediaModule {
    pub fn new() -> Self {
        Self { enabled: false }
    }

    fn find_player(&self) -> Option<String> {
        let output = Command::new("playerctl").args(["-l"]).output().ok()?;
        let list = String::from_utf8_lossy(&output.stdout);

        for preferred in PREFERRED_PLAYERS {
            for line in list.lines() {
                if line.starts_with(preferred) {
                    return Some(line.trim().to_string());
                }
            }
        }

        list.lines()
            .next()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    }

    fn query(&self, player: &str) -> Option<MediaInfo> {
        let output = Command::new("playerctl")
            .args([
                "-p",
                player,
                "metadata",
                "--format",
                "{{artist}}\n{{title}}\n{{status}}\n{{position}}\n{{mpris:length}}",
            ])
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let text = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = text.lines().collect();
        if lines.len() < 5 {
            return None;
        }

        Some(MediaInfo {
            artist: lines[0].to_string(),
            title: lines[1].to_string(),
            status: lines[2].to_string(),
            position_us: lines[3].parse().unwrap_or(0),
            length_us: lines[4].parse().unwrap_or(0),
        })
    }
}

fn format_time(us: u64) -> String {
    let total_secs = us / 1_000_000;
    let mins = total_secs / 60;
    let secs = total_secs % 60;
    format!("{mins}:{secs:02}")
}

fn progress_bar(position: u64, length: u64) -> String {
    if length == 0 {
        return "\u{2015}".repeat(BAR_LENGTH);
    }

    let fraction = position as f64 / length as f64;
    let filled = (fraction * BAR_LENGTH as f64).round() as usize;
    let filled = filled.min(BAR_LENGTH);
    let empty = BAR_LENGTH - filled;

    format!("{}{}", "\u{2501}".repeat(filled), "\u{2015}".repeat(empty))
}

impl Module for MediaModule {
    fn name(&self) -> &str {
        "Media"
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

        let player = self.find_player()?;
        let info = self.query(&player)?;

        if info.artist.is_empty() && info.title.is_empty() {
            return None;
        }

        let icon = match info.status.as_str() {
            "Playing" => "\u{25B6}",
            "Paused" => "\u{23F8}",
            _ => "\u{25A0}",
        };

        let bar = progress_bar(info.position_us, info.length_us);
        let pos = format_time(info.position_us);
        let dur = format_time(info.length_us);

        Some(format!(
            "{icon} {} - {}\n{bar} {pos}/{dur}",
            info.artist, info.title
        ))
    }
}
