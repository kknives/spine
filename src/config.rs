use figment::{Figment, providers::{Format, Toml}};
use serde::Deserialize;
use std::collections::HashMap;
use crate::server::HardwareRequest;
use tracing::debug;

#[derive(Default, Deserialize, Debug)]
pub struct PadConfig {
    motors: HashMap<String, u8>,
    encoders: HashMap<String, u8>,
}
#[derive(Deserialize, Debug)]
pub struct SystemConfig {
    motors: HashMap<String, u8>,
}
#[derive(Deserialize, Debug)]
pub struct Config {
    pad: PadConfig,
    system: SystemConfig,
}
pub enum Handler {
    Pad(u8),
    System(u8),
}

impl Config {
    pub fn resolve(&self, hrq: &HardwareRequest) -> Option<Handler> {
        match hrq {
            HardwareRequest::MotorWrite{motor, command} => {
                self.pad.motors.get(motor).map(|port| Handler::Pad(*port)).or_else(|| self.system.motors.get(motor).map(|port| Handler::System(*port)))
            }
            HardwareRequest::EncoderRead{encoder} => {
                self.pad.encoders.get(encoder).map(|port| Handler::Pad(*port))
            }
        }
    }
}
#[tracing::instrument]
pub fn load_config() -> Config {
    // Fix this
    let config = Figment::new()
        .merge(Toml::file("config.toml").nested());
    let pad_config: PadConfig = config.select("pad").extract().unwrap();
    let config = Figment::new()
        .merge(Toml::file("config.toml").nested());
    let system_config: SystemConfig = config.select("system").extract().unwrap();
    let config = Config {
        pad: pad_config,
        system: system_config,
    };
    // let pad_config: SystemConfig = config.select("system").extract().unwrap();
    // println!("{:#?}", pad_config);
    debug!("{:#?}", config);
    config
}
