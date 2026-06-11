mod config;
mod logger;
mod ollama;

use colored::*;

#[tokio::main]
async fn main() {
    let cfg = config::config_oku();

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(cfg.system.timeout_sn))
        .build()
        .unwrap();

    let tarih = chrono::Local::now().format("%Y-%m-%d_%H-%M").to_string();

    println!();
    println!("{}", "╔══════════════════════════════════════╗".blue());
    println!("{}", "║     RUST MULTI-AGENT SİSTEMİ         ║".blue());
    println!("{}", "╚══════════════════════════════════════╝".blue());
    println!();
    println!("{} {}", "📌 Konu:".bold(), cfg.scenario.konu.yellow());
    println!("{} {}", "🔄 Tur sayısı:".bold(), cfg.scenario.tur_sayisi);
    println!("{} {}", "🌐 Host:".bold(), cfg.system.host.cyan());
    println!("{} {} sn", "⏱ Timeout:".bold(), cfg.system.timeout_sn);
    println!(
        "{} {}",
        "💾 Log:".bold(),
        format!("logs/{}.json", tarih).cyan()
    );
    println!();

    let mut gecmis = String::new();
    let mut analizler: Vec<String> = Vec::new();
    let mut tur_loglari: Vec<logger::TurLog> = Vec::new();

    for tur in 1..=cfg.scenario.tur_sayisi {
        println!(
            "{}",
            format!("━━━ TUR {} ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━", tur).bold()
        );
        println!();

        // ── AI-A konuşuyor ───────────────────────────────────
        let a_prompt = format!(
            "{}\n\nKonu: {}\n\nŞimdiye kadarki konuşma:\n{}\n\nŞimdi sen konuş. Kısa ve güçlü argüman kur (3-4 cümle):",
            cfg.agents.a_rol, cfg.scenario.konu, gecmis
        );

        print!("{} ", format!("[{}]", cfg.agents.a_isim).blue().bold());
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let (a_yanit, a_token, a_gecikme) =
            ollama::ask(&client, &cfg.system.host, &cfg.agents.a_model, &a_prompt).await;

        println!("{}", a_yanit.blue());
        println!(
            "{}",
            format!("   ↳ {} token | {} ms", a_token, a_gecikme).dimmed()
        );
        println!();

        gecmis.push_str(&format!("{}: {}\n\n", cfg.agents.a_isim, a_yanit));

        // ── AI-B konuşuyor ───────────────────────────────────
        let b_prompt = format!(
            "{}\n\nKonu: {}\n\nŞimdiye kadarki konuşma:\n{}\n\nŞimdi sen konuş. Kısa ve güçlü argüman kur (3-4 cümle):",
            cfg.agents.b_rol, cfg.scenario.konu, gecmis
        );

        print!("{} ", format!("[{}]", cfg.agents.b_isim).green().bold());
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let (b_yanit, b_token, b_gecikme) =
            ollama::ask(&client, &cfg.system.host, &cfg.agents.b_model, &b_prompt).await;

        println!("{}", b_yanit.green());
        println!(
            "{}",
            format!("   ↳ {} token | {} ms", b_token, b_gecikme).dimmed()
        );
        println!();

        gecmis.push_str(&format!("{}: {}\n\n", cfg.agents.b_isim, b_yanit));

        // ── AI-C analiz ediyor (arka planda) ─────────────────
        let c_prompt = format!(
            "{}\n\nTur {} konuşması:\n{}: {}\n{}: {}\n\nAnalizini yap:",
            cfg.agents.c_rol, tur, cfg.agents.a_isim, a_yanit, cfg.agents.b_isim, b_yanit
        );

        let (c_yanit, c_token, c_gecikme) =
            ollama::ask(&client, &cfg.system.host, &cfg.agents.c_model, &c_prompt).await;

        analizler.push(format!("Tur {}: {}", tur, c_yanit));
        gecmis.push_str(&format!("Gözlemci Analizi: {}\n\n", c_yanit));

        tur_loglari.push(logger::TurLog {
            tur,
            a_yanit,
            a_token,
            a_gecikme_ms: a_gecikme,
            b_yanit,
            b_token,
            b_gecikme_ms: b_gecikme,
            analiz: c_yanit,
            c_token,
            c_gecikme_ms: c_gecikme,
        });
    }

    // ── Analizleri göster ─────────────────────────────────────
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

    // ── Metrik özeti ──────────────────────────────────────────
    println!("{}", "╔══════════════════════════════════════╗".cyan());
    println!("{}", "║         METRİK ÖZET                  ║".cyan());
    println!("{}", "╚══════════════════════════════════════╝".cyan());
    println!();

    let a_toplam_token: u64 = tur_loglari.iter().map(|t| t.a_token).sum();
    let b_toplam_token: u64 = tur_loglari.iter().map(|t| t.b_token).sum();
    let a_ort_gecikme =
        tur_loglari.iter().map(|t| t.a_gecikme_ms).sum::<u128>() / tur_loglari.len() as u128;
    let b_ort_gecikme =
        tur_loglari.iter().map(|t| t.b_gecikme_ms).sum::<u128>() / tur_loglari.len() as u128;

    println!(
        "{} toplam {} token | ortalama {} ms/yanıt",
        format!("[{}]", cfg.agents.a_isim).blue().bold(),
        a_toplam_token.to_string().cyan(),
        a_ort_gecikme.to_string().cyan()
    );
    println!(
        "{} toplam {} token | ortalama {} ms/yanıt",
        format!("[{}]", cfg.agents.b_isim).green().bold(),
        b_toplam_token.to_string().cyan(),
        b_ort_gecikme.to_string().cyan()
    );
    println!();

    // ── Genel özet ────────────────────────────────────────────
    println!("{}", "╔══════════════════════════════════════╗".cyan());
    println!("{}", "║         GENEL ÖZET                   ║".cyan());
    println!("{}", "╚══════════════════════════════════════╝".cyan());
    println!();

    let ozet_prompt = format!(
        "Sen tarafsız bir analistsin. Aşağıdaki müzakere konuşmasını analiz et:\n\n{}\n\nŞunları söyle:\n1) Genel kazanan kim ve neden?\n2) Her tarafın en güçlü ve en zayıf anı\n3) Kullanılan ikna teknikleri\n4) Final skoru: {} kaç/10, {} kaç/10",
        gecmis, cfg.agents.a_isim, cfg.agents.b_isim
    );

    print!("{}", "Genel özet hazırlanıyor...".cyan());
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    let (ozet, _, _) =
        ollama::ask(&client, &cfg.system.host, &cfg.agents.c_model, &ozet_prompt).await;
    println!("\r{}", " ".repeat(40));
    println!("{}", ozet.cyan());

    // ── Log kaydet ────────────────────────────────────────────
    let oturum = logger::OturumLog {
        konu: cfg.scenario.konu.clone(),
        tarih: tarih.clone(),
        host: cfg.system.host.clone(),
        timeout_sn: cfg.system.timeout_sn,
        toplam_tur: cfg.scenario.tur_sayisi,
        turlar: tur_loglari,
    };

    logger::kaydet(&oturum, &tarih);
}
