pub mod afk;
pub mod heartrate;
pub mod media;
pub mod network;
pub mod stats;
pub mod status;
pub mod system_details;
pub mod time;

pub trait Module: Send {
    fn name(&self) -> &str;
    fn enabled(&self) -> bool;
    fn set_enabled(&mut self, enabled: bool);
    fn tick(&mut self) -> Option<String>;
}
