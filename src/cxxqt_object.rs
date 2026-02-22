#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, osc_address)]
        #[qproperty(bool, status_enabled)]
        #[qproperty(QString, status_line1)]
        #[qproperty(QString, status_line2)]
        #[qproperty(QString, status_line3)]
        #[qproperty(QString, status_line4)]
        #[qproperty(QString, status_line5)]
        #[qproperty(QString, status_line6)]
        #[qproperty(bool, time_enabled)]
        #[qproperty(QString, time_format)]
        #[qproperty(bool, stats_enabled)]
        #[qproperty(bool, stats_show_cpu)]
        #[qproperty(bool, stats_show_ram)]
        #[qproperty(bool, network_enabled)]
        #[qproperty(bool, media_enabled)]
        #[qproperty(bool, afk_enabled)]
        #[qproperty(i32, afk_timeout_secs)]
        #[qproperty(QString, afk_text)]
        #[qproperty(bool, system_details_enabled)]
        #[qproperty(bool, heartrate_enabled)]
        #[qproperty(QString, heartrate_token)]
        #[qproperty(QString, last_output)]
        #[qproperty(bool, running)]
        #[namespace = "osc_chatbox"]
        type ChatboxController = super::ChatboxControllerRust;

        #[qinvokable]
        fn start(self: Pin<&mut Self>);

        #[qinvokable]
        fn stop(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "sendMessage"]
        fn send_message(self: Pin<&mut Self>, text: &QString);

        #[qinvokable]
        #[cxx_name = "setTyping"]
        fn set_typing(self: Pin<&mut Self>, typing: bool);

        #[qinvokable]
        fn tick(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "applySettings"]
        fn apply_settings(self: Pin<&mut Self>);
    }
}

use core::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;

use crate::modules::Module;
use crate::orchestrator::Orchestrator;

pub struct ChatboxControllerRust {
    osc_address: QString,
    status_enabled: bool,
    status_line1: QString,
    status_line2: QString,
    status_line3: QString,
    status_line4: QString,
    status_line5: QString,
    status_line6: QString,
    time_enabled: bool,
    time_format: QString,
    stats_enabled: bool,
    stats_show_cpu: bool,
    stats_show_ram: bool,
    network_enabled: bool,
    media_enabled: bool,
    afk_enabled: bool,
    afk_timeout_secs: i32,
    afk_text: QString,
    system_details_enabled: bool,
    heartrate_enabled: bool,
    heartrate_token: QString,
    last_output: QString,
    running: bool,
    orchestrator: Option<Orchestrator>,
}

impl Default for ChatboxControllerRust {
    fn default() -> Self {
        Self {
            osc_address: QString::from("127.0.0.1:9000"),
            status_enabled: false,
            status_line1: QString::from(""),
            status_line2: QString::from(""),
            status_line3: QString::from(""),
            status_line4: QString::from(""),
            status_line5: QString::from(""),
            status_line6: QString::from(""),
            time_enabled: false,
            time_format: QString::from("%H:%M"),
            stats_enabled: false,
            stats_show_cpu: true,
            stats_show_ram: true,
            network_enabled: false,
            media_enabled: false,
            afk_enabled: false,
            afk_timeout_secs: 300,
            afk_text: QString::from("AFK"),
            system_details_enabled: false,
            heartrate_enabled: false,
            heartrate_token: QString::from(""),
            last_output: QString::from(""),
            running: false,
            orchestrator: None,
        }
    }
}

fn get_inner(pin: Pin<&mut ChatboxControllerRust>) -> &mut ChatboxControllerRust {
    unsafe { pin.get_unchecked_mut() }
}

impl qobject::ChatboxController {
    pub fn start(mut self: Pin<&mut Self>) {
        let addr = self.osc_address().to_string();
        match Orchestrator::new(&addr) {
            Ok(orch) => {
                let inner = get_inner(self.as_mut().rust_mut());
                inner.orchestrator = Some(orch);
                self.as_mut().set_running(true);
                self.apply_settings();
            }
            Err(e) => {
                self.as_mut()
                    .set_last_output(QString::from(&format!("Error: {e}")));
            }
        }
    }

    pub fn stop(mut self: Pin<&mut Self>) {
        let inner = get_inner(self.as_mut().rust_mut());
        inner.orchestrator = None;
        self.as_mut().set_running(false);
    }

    pub fn send_message(self: Pin<&mut Self>, text: &QString) {
        let text_str = text.to_string();
        let inner = get_inner(self.rust_mut());
        if let Some(ref mut orch) = inner.orchestrator {
            orch.set_chat_message(text_str);
        }
    }

    pub fn set_typing(self: Pin<&mut Self>, typing: bool) {
        let inner = get_inner(self.rust_mut());
        if let Some(ref orch) = inner.orchestrator {
            let _ = orch.send_typing(typing);
        }
    }

    pub fn tick(self: Pin<&mut Self>) {
        let inner = get_inner(self.rust_mut());
        if let Some(ref mut orch) = inner.orchestrator {
            let _ = orch.tick();
        }
    }

    pub fn apply_settings(self: Pin<&mut Self>) {
        let inner = get_inner(self.rust_mut());
        let status_enabled = inner.status_enabled;
        let status_lines: [String; 6] = [
            inner.status_line1.to_string(),
            inner.status_line2.to_string(),
            inner.status_line3.to_string(),
            inner.status_line4.to_string(),
            inner.status_line5.to_string(),
            inner.status_line6.to_string(),
        ];
        let time_enabled = inner.time_enabled;
        let time_format = inner.time_format.to_string();
        let stats_enabled = inner.stats_enabled;
        let stats_show_cpu = inner.stats_show_cpu;
        let stats_show_ram = inner.stats_show_ram;
        let network_enabled = inner.network_enabled;
        let media_enabled = inner.media_enabled;
        let afk_enabled = inner.afk_enabled;
        let afk_timeout = inner.afk_timeout_secs;
        let afk_text = inner.afk_text.to_string();
        let system_details_enabled = inner.system_details_enabled;
        let heartrate_enabled = inner.heartrate_enabled;
        let heartrate_token = inner.heartrate_token.to_string();

        if let Some(ref mut orch) = inner.orchestrator {
            orch.status.set_enabled(status_enabled);
            for (i, line) in status_lines.iter().enumerate() {
                orch.status.set_line(i, line.clone());
            }
            orch.time.set_enabled(time_enabled);
            orch.time.set_format(time_format);
            orch.stats.set_enabled(stats_enabled);
            orch.stats.set_show_cpu(stats_show_cpu);
            orch.stats.set_show_ram(stats_show_ram);
            orch.network.set_enabled(network_enabled);
            orch.media.set_enabled(media_enabled);
            orch.afk.set_enabled(afk_enabled);
            orch.afk.set_timeout_secs(afk_timeout as u64);
            orch.afk.set_afk_text(afk_text);
            orch.system_details.set_enabled(system_details_enabled);
            orch.heartrate.set_enabled(heartrate_enabled);
            orch.heartrate.set_token(heartrate_token);
        }
    }
}
