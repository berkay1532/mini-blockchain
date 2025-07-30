# 🧱 mini_blockchain

Rust ile yazılmış, modüler ve öğrenmeye yönelik bir **blockchain uygulaması**.

Bu proje, Rust dilinin memory safety, ownership, concurrency ve modüler yapısını kullanarak bir blockchain'in temellerini kavramayı ve farklı senaryolarda uygulamayı amaçlar.

---

## 📁 Proje Yapısı

```bash
mini_blockchain/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Ortak modüller
│   ├── block.rs
│   ├── blockchain.rs
│   ├── transaction.rs
│   ├── pow.rs
│   ├── server.rs
│   └── bin/
│       ├── basic.rs        # Basit terminal demo
│       ├── cli.rs   # CLI + dosya kalıcılığı
│       └── web.rs  # HTTP sunucu (REST API)
```

## 🔧 Kurulum

git clone https://github.com/kullaniciadi/mini_blockchain.git
cd mini_blockchain
cargo build

# 🚀 Kullanım Senaryoları

1. 🖥️ basic.rs – Temel terminal uygulaması

```bash
cargo run --bin basic
```

Blockchain doğrudan kod içinde tanımlanır

İşlemler println! ile gösterilir

Eğitim ve test amaçlıdır

2. 💻 cli.rs – Komut satırı uygulaması

```bash
cargo run --bin cli -- add --from Alice --to Bob --amount 50
cargo run --bin cli -- show
cargo run --bin cli -- validate
cargo run --bin cli -- reset --difficulty 4
```

clap kütüphanesi ile CLI arayüz

Zincir blockchain.json dosyasına kaydedilir

Zincir diskten yüklenir, kalıcıdır

3. 🌐 server.rs – HTTP sunucu (REST API)

```bash
cargo run --bin server
```

| Method | Path        | Açıklama                     |
| ------ | ----------- | ---------------------------- |
| GET    | `/chain`    | Tüm blockchain'i döner       |
| POST   | `/add-tx`   | Yeni işlem gönderir          |
| GET    | `/validate` | Zincirin geçerliliğini döner |

curl -X POST http://localhost:3030/add-tx \
 -H "Content-Type: application/json" \
 -d '{"sender": "Ali", "receiver": "Veli", "amount": 100}'
