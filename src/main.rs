use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize)]
struct Config {
    scenario: Scenario,
    agents: Agents,
}

#[derive(Deserialize)]
struct Scenario {
    konu: String,
    tur_sayisi: u32,
}

#[derive(Deserialize)]
struct Agents {
    a_model: String,
    a_isim: String,
    a_rol: String,
    b_model: String,
    b_isim: String,
    b_rol: String,
    c_model: String,
    c_rol: String,
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

#[derive(Serialize)]
struct TurLog {
    tur: u32,
    a_yanit: String,
    b_yanit: String,
    analiz: String,
}

#[derive(Serialize)]
struct OturumLog {
    konu: String,
    tarih: String,
    turlar: Vec<TurLog>,
}

async fn ask(client: &reqwest::Client, model: &str, prompt: &str) -> String {
    let req = OllamaRequest {
        model: model.to_string(),
        prompt: prompt.to_string(),
        stream: false,
    };

    let res = client
        .post("http://localhost:11434/api/generate")
        .json(&req)
        .send()
        .await
        .expect("Ollama'ya bağlanılamadı");

    let body: OllamaResponse = res.json().await.expect("Yanıt parse edilemedi");
    body.response.trim().to_string()
}

#[tokio::main]
async fn main() {
    let config_str = fs::read_to_string("config.toml").expect("config.toml bulunamadı");
    let config: Config = toml::from_str(&config_str).expect("config.toml okunamadı");

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .unwrap();

    // Tarih oluştur
    let tarih = chrono::Local::now().format("%Y-%m-%d_%H-%M").to_string();

    println!();
    println!("{}", "╔══════════════════════════════════════╗".blue());
    println!("{}", "║     RUST MULTI-AGENT SİSTEMİ         ║".blue());
    println!("{}", "╚══════════════════════════════════════╝".blue());
    println!();
    println!("{} {}", "📌 Konu:".bold(), config.scenario.konu.yellow());
    println!("{} {}", "🔄 Tur sayısı:".bold(), config.scenario.tur_sayisi);
    println!("{} {}", "💾 Log dosyası:".bold(), format!("logs/{}.json", tarih).cyan());
    println!();

    let mut gecmis = String::new();
    let mut analizler: Vec<String> = Vec::new();
    let mut tur_loglari: Vec<TurLog> = Vec::new();

    for tur in 1..=config.scenario.tur_sayisi {
        println!("{}", format!("━━━ TUR {} ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━", tur).bold());
        println!();

        // ── AI-A konuşuyor ───────────────────────────────────
        let a_prompt = format!(
            "{}\n\nKonu: {}\n\nŞimdiye kadarki konuşma:\n{}\n\nŞimdi sen konuş. Kısa ve güçlü argüman kur (3-4 cümle):",
            config.agents.a_rol,
            config.scenario.konu,
            gecmis
        );

        print!("{} ", format!("[{}]", config.agents.a_isim).blue().bold());
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        let a_yanit = ask(&client, &config.agents.a_model, &a_prompt).await;
        println!("{}", a_yanit.blue());
        println!();

        gecmis.push_str(&format!("{}: {}\n\n", config.agents.a_isim, a_yanit));

        // ── AI-B konuşuyor ───────────────────────────────────
        let b_prompt = format!(
            "{}\n\nKonu: {}\n\nŞimdiye kadarki konuşma:\n{}\n\nŞimdi sen konuş. Kısa ve güçlü argüman kur (3-4 cümle):",
            config.agents.b_rol,
            config.scenario.konu,
            gecmis
        );

        print!("{} ", format!("[{}]", config.agents.b_isim).green().bold());
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        let b_yanit = ask(&client, &config.agents.b_model, &b_prompt).await;
        println!("{}", b_yanit.green());
        println!();

        gecmis.push_str(&format!("{}: {}\n\n", config.agents.b_isim, b_yanit));

        // ── AI-C analiz ediyor (arka planda) ─────────────────
        let c_prompt = format!(
            "{}\n\nTur {} konuşması:\n{}: {}\n{}: {}\n\nAnalizini yap:",
            config.agents.c_rol,
            tur,
            config.agents.a_isim,
            a_yanit,
            config.agents.b_isim,
            b_yanit
        );

        let c_yanit = ask(&client, &config.agents.c_model, &c_prompt).await;
        analizler.push(format!("Tur {}: {}", tur, c_yanit));
        gecmis.push_str(&format!("Gözlemci Analizi: {}\n\n", c_yanit));

        // ── Log kaydı ─────────────────────────────────────────
        tur_loglari.push(TurLog {
            tur,
            a_yanit,
            b_yanit,
            analiz: c_yanit,
        });
    }

    // ── Tüm turlar bitti, analizleri göster ──────────────────
    println!("{}", "╔══════════════════════════════════════╗".bold());
    println!("{}", "║         KONUŞMA TAMAMLANDI           ║".bold());
    println!("{}", "╚══════════════════════════════════════╝".bold());
    println!();
    println!("{}", "╔══════════════════════════════════════╗".yellow());
    println!("{}", "║         ANALİZ RAPORU                ║".yellow());
    println!("{}", "╚══════════════════════════════════════╝".yellow());
    println!();

    for analiz in &analizler {
        println!("{}", analiz.yellow());
        println!();
    }

    // ── JSON log dosyasına kaydet ─────────────────────────────
    let oturum = OturumLog {
        konu: config.scenario.konu.clone(),
        tarih: tarih.clone(),
        turlar: tur_loglari,
    };

    fs::create_dir_all("logs").unwrap();
    let log_yolu = format!("logs/{}.json", tarih);
    let json = serde_json::to_string_pretty(&oturum).unwrap();
    fs::write(&log_yolu, json).unwrap();

    println!("{} {}", "💾 Log kaydedildi:".bold().cyan(), log_yolu.cyan());
}