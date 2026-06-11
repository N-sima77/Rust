use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub scenario: Scenario,
    pub system: System,
    pub agents: Agents,
}

#[derive(Deserialize)]
pub struct Scenario {
    pub konu: String,
    pub tur_sayisi: u32,
}

#[derive(Deserialize)]
pub struct System {
    pub timeout_sn: u64,
    pub host: String,
}

#[derive(Deserialize)]
pub struct Agents {
    pub a_model: String,
    pub a_isim: String,
    pub a_rol: String,
    pub b_model: String,
    pub b_isim: String,
    pub b_rol: String,
    pub c_model: String,
    pub c_rol: String,
}

pub fn config_oku() -> Config {
    let icerik = fs::read_to_string("config.toml").expect("config.toml bulunamadı");
    toml::from_str(&icerik).expect("config.toml okunamadı")
}
