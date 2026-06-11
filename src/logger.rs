use serde::Serialize;
use std::fs;

#[derive(Serialize)]
pub struct TurLog {
    pub tur: u32,
    pub a_yanit: String,
    pub a_token: u64,
    pub a_gecikme_ms: u128,
    pub b_yanit: String,
    pub b_token: u64,
    pub b_gecikme_ms: u128,
    pub analiz: String,
    pub c_token: u64,
    pub c_gecikme_ms: u128,
}

#[derive(Serialize)]
pub struct OturumLog {
    pub konu: String,
    pub tarih: String,
    pub host: String,
    pub timeout_sn: u64,
    pub toplam_tur: u32,
    pub turlar: Vec<TurLog>,
}

pub fn kaydet(oturum: &OturumLog, tarih: &str) {
    fs::create_dir_all("logs").unwrap();
    let log_yolu = format!("logs/{}.json", tarih);
    let json = serde_json::to_string_pretty(oturum).unwrap();
    fs::write(&log_yolu, json).unwrap();
    println!();
    println!("💾 Log kaydedildi: {}", log_yolu);
}
