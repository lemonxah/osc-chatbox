use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscType};
use std::net::UdpSocket;

pub struct OscClient {
    socket: UdpSocket,
    target: String,
}

impl OscClient {
    pub fn new(target_addr: &str) -> Result<Self, std::io::Error> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        Ok(Self {
            socket,
            target: target_addr.to_string(),
        })
    }

    pub fn send_chatbox_message(
        &self,
        text: &str,
        send_immediately: bool,
        play_sound: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut truncated: String = text.chars().take(142).collect();
        truncated.push_str("\u{0003}\u{001F}");

        let msg = OscPacket::Message(OscMessage {
            addr: "/chatbox/input".to_string(),
            args: vec![
                OscType::String(truncated),
                OscType::Bool(send_immediately),
                OscType::Bool(play_sound),
            ],
        });

        let buf = encoder::encode(&msg)?;
        self.socket.send_to(&buf, &self.target)?;
        Ok(())
    }

    pub fn send_typing_indicator(&self, is_typing: bool) -> Result<(), Box<dyn std::error::Error>> {
        let msg = OscPacket::Message(OscMessage {
            addr: "/chatbox/typing".to_string(),
            args: vec![OscType::Bool(is_typing)],
        });

        let buf = encoder::encode(&msg)?;
        self.socket.send_to(&buf, &self.target)?;
        Ok(())
    }
}
