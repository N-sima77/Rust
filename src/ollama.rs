use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
    eval_count: Option<u64>,
}

pub async fn ask(
    client: &reqwest::Client,
    host: &str,
    model: &str,
    prompt: &str,
) -> (String, u64, u128) {
    let req = OllamaRequest {
        model: model.to_string(),
        prompt: prompt.to_string(),
        stream: false,
    };

    let baslangic = Instant::now();

    let res = client
        .post(format!("{}/api/generate", host))
        .json(&req)
        .send()
        .await
        .expect("Ollama'ya bağlanılamadı");

    let body: OllamaResponse = res.json().await.expect("Yanıt parse edilemedi");

    let gecikme = baslangic.elapsed().as_millis();
    let token = body.eval_count.unwrap_or(0);

    (body.response.trim().to_string(), token, gecikme)
}
