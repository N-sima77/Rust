# Rust Multi-Agent Yapay Zeka Sistemi

Rust ile yazılmış,çevrimdışı çalışan çok-ajanlı yapay zeka sohbet sistemi.
İki yapay zeka modeli bağımsız bir analistı ikna etmeye çalışıyor, analist her turu değerlendiriyor.
Tüm modeller yerel makinede Ollama üzerinden çalışır, internet gerekmiyor.

---

## Proje Hakkında

Bu proje, Programlama dilleri dersi kapsamında geliştirilmiştir.

### Sistem Nasıl Çalışır?

- **AI-A** (llama3.1:8b) — örnek olarak 1. şirket temsilcisi, analistı ikna etmeye çalışır
- **AI-B** (mistral:7b) — örnek olarak 2. şirket temsilcisi, analistı ikna etmeye çalışır  
- **AI-C** (qwen2.5:7b) — bağımsız analist, her tur değerlendirme yapar, turlar bitince final kararını açıklar

Belirnenen tur boyunca yapay zekalar diyolog halinde konuşurlar.Her tur sonunda AI-C'nin analizi bir sonraki turda AI-A ve AI-B'ye iletilir.
Modeller birbirini değil, 3. yapay zekayı gözlemciyi,analisti ikna etmeye çalışır.
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
git clone [https://github.com/N-sima77/Rust.git]
cd rust_project
```

### 5. Derleyin

```bash
cargo build
```

---

## Kullanım

### Çalıştırmak için

```bash
cargo run
```

### Senaryo Değiştirme için

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

╔══════════════════════════════════════╗
║     RUST MULTI-AGENT SİSTEMİ         ║
╚══════════════════════════════════════╝
Örnek:

📌 Konu: İki şirket aynı büyük yazılım projesini almak için müzakere ediyor
🔄 Tur sayısı: 2

━━━ TUR 1 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[A Şirketi] Projemizin başarı oranı sektör ortalamasının iki katı...
   ↳ 287 token | 4823 ms

[B Şirketi] A Şirketi'nin iddialarının aksine...
   ↳ 312 token | 5102 ms
.
.
.
