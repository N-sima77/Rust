# Rust Multi-Agent Yapay Zeka Sistemi

Rust ile yazılmış,çevrimdışı çalışan çok-ajanlı yapay zeka sistemi.
İki yapay zeka modeli bağımsız bir analist tarafındna analiz ediliyor.
Tüm modeller yerel makinede Ollama üzerinden çalışır, internet gerekmez.

---

## Proje Hakkında

Bu proje, Programlama dilleri dersi kapsamında geliştirilmiştir.

### Sistem Nasıl Çalışır?

- **AI-A** (llama3.1:8b) — örnek olarak 1. şirket temsilcisi, analistı ikna etmeye çalışır
- **AI-B** (mistral:7b) — örnek olarak 2. şirket temsilcisi, analistı ikna etmeye çalışır  
- **AI-C** (qwen2.5:7b) — bağımsız analist, her tur değerlendirme yapar, turlar bitince final kararını açıklar

Her tur sonunda AI-C'nin analizi bir sonraki turda AI-A ve AI-B'ye iletilir.
Modeller birbirini değil, analistı ikna etmeye çalışır.
Tüm konuşma `logs/` klasörüne JSON olarak kaydedilir.

---

## Kullanılan Teknolojiler

| Teknoloji | Versiyon | Kullanım Amacı |
|---|---|---|
| Rust | 1.78+ | Ana programlama dili |
| tokio | 1.x | Async runtime |
| reqwest | 0.12 | Ollama HTTP API istemcisi |
| serde / serde_json | 1.x | JSON parse ve loglama |
| toml | 0.8 | Konfigürasyon dosyası okuma |
| chrono | 0.4 | Log dosyası tarih damgası |
| colored | 2.x | Renkli terminal çıktısı |
| Ollama | latest | Yerel LLM çalıştırma |

---

## Kurulum

### Gereksinimler

- Windows / macOS / Linux
- Rust (rustup ile kurulum önerilir)
- Ollama
- 32 GB RAM önerilir (minimum 16 GB)
- ~14 GB disk alanı (3 model için)

### 1. Rust Kurulumu

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Windows için: https://rustup.rs adresinden indirin.

### 2. Ollama Kurulumu

https://ollama.com/download adresinden indirin ve kurun.

### 3. Modelleri İndirin

```bash
ollama pull llama3.1:8b
ollama pull mistral:7b
ollama pull qwen2.5:7b
```

### 4. Projeyi Klonlayın

```bash
git clone https://github.com/kullaniciadı/Rust.git
cd rust-multi-agent
```

### 5. Derleyin

```bash
cargo build
```

---

## Kullanım

### Çalıştırma

```bash
cargo run
```

### Senaryo Değiştirme

`config.toml` dosyasını düzenleyin:

```toml
[scenario]
konu = "İki şirket aynı büyük yazılım projesini almak için müzakere ediyor"
tur_sayisi = 6

[agents]
a_isim = "A Şirketi"
b_isim = "B Şirketi"
a_model = "llama3.1:8b"
a_rol = "Sen A Şirketi temsilcisisin..."
b_model = "mistral:7b"
b_rol = "Sen B Şirketi temsilcisisin..."
c_model = "qwen2.5:7b"
c_rol = "Sen bağımsız bir analistsin..."
```

Kodu değiştirmeden farklı senaryolar denenebilir:
- İki avukat — hakim ikna
- İki startup — yatırımcı ikna

### Terminal Çıktısı
