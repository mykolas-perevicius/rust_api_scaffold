# Rust API Scaffold

[![Rust CI](https://img.shields.io/github/actions/workflow/status/your-handle/rust_api_scaffold/rust.yml?branch=main)](../../actions)  
Starter **Actix-Web** + **SQLx/Postgres** service.

## ✨ Features
* Actix-Web 4 health-check route (`GET /health`)  
* Async Postgres ready via SQLx  
* Docker-friendly; binds to `0.0.0.0:8080`  
* CI workflow builds on every push

## 🚀 Quick start
```bash
git clone https://github.com/your-handle/rust_api_scaffold
cd rust_api_scaffold
cargo run          # http://localhost:8080/health → {"status":"ok"}
```

## 🗄  Database

1. `sudo apt install postgresql`
2. `createdb scaffold_db`
3. Put `DATABASE_URL=postgres://USER:PASS@localhost/scaffold_db` in `.env`
4. Add migrations, then

   ```bash
   cargo sqlx migrate run
   ```

## 🧪 Test & lint

```bash
cargo test        # unit tests
cargo clippy -W clippy::all
cargo fmt --check
```

## 📦 Release build

```bash
cargo build --release
```

## 🤝 Contributing

Issues and PRs welcome! Please run `cargo fmt` before committing.

## 📜 License

MIT © 2025 Mykolas Perevicius
[5]: https://github.com/rust-lang/rustfmt?utm_source=chatgpt.com "rust-lang/rustfmt: Format Rust code - GitHub"
