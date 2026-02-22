use std::time::{Duration, Instant};

use crate::modules::afk::AfkModule;
use crate::modules::heartrate::HeartrateModule;
use crate::modules::media::MediaModule;
use crate::modules::network::NetworkModule;
use crate::modules::stats::StatsModule;
use crate::modules::status::StatusModule;
use crate::modules::system_details::SystemDetailsModule;
use crate::modules::time::TimeModule;
use crate::modules::Module;
use crate::osc::OscClient;

const CHAT_MESSAGE_DURATION: Duration = Duration::from_secs(20);

pub struct Orchestrator {
    osc: OscClient,
    pub status: StatusModule,
    pub time: TimeModule,
    pub stats: StatsModule,
    pub network: NetworkModule,
    pub media: MediaModule,
    pub afk: AfkModule,
    pub heartrate: HeartrateModule,
    pub system_details: SystemDetailsModule,
    pending_chat_message: Option<(String, Instant)>,
}

impl Orchestrator {
    pub fn new(osc_target: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            osc: OscClient::new(osc_target)?,
            status: StatusModule::new(),
            time: TimeModule::new(),
            stats: StatsModule::new(),
            network: NetworkModule::new(),
            media: MediaModule::new(),
            afk: AfkModule::new(),
            heartrate: HeartrateModule::new(),
            system_details: SystemDetailsModule::new(),
            pending_chat_message: None,
        })
    }

    pub fn set_chat_message(&mut self, text: String) {
        self.pending_chat_message = Some((text, Instant::now()));
    }

    pub fn send_typing(&self, typing: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.osc.send_typing_indicator(typing)
    }

    pub fn tick(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Check if the pending chat message has expired
        if let Some((_, sent_at)) = &self.pending_chat_message {
            if sent_at.elapsed() >= CHAT_MESSAGE_DURATION {
                self.pending_chat_message = None;
            }
        }

        let modules: Vec<&mut dyn Module> = vec![
            &mut self.status,
            &mut self.time,
            &mut self.stats,
            &mut self.network,
            &mut self.media,
            &mut self.heartrate,
            &mut self.system_details,
            &mut self.afk,
        ];

        let parts: Vec<String> = modules.into_iter().filter_map(|m| m.tick()).collect();
        let module_line = parts.join("\n");

        // Build the final message: chat message on top, module output below
        let play_sound = self.pending_chat_message.is_some();
        let combined = match &self.pending_chat_message {
            Some((msg, _)) if !module_line.is_empty() => {
                format!("{msg}\n{module_line}")
            }
            Some((msg, _)) => msg.clone(),
            None if !module_line.is_empty() => module_line,
            None => return Ok(()),
        };

        self.osc.send_chatbox_message(&combined, true, play_sound)
    }
}
