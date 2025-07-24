# 🔐 Rust Secure Password Generator CLI

A simple, secure command-line tool written in Rust to generate random passwords using the [`rand`](https://crates.io/crates/rand) crate.

## 🚀 Features

- Secure, random password generation
- Customizable password length via command-line argument
- Fast and memory-safe (thanks to Rust!)
- Optional: Easily extendable to support symbols, clipboard copy, etc.

---

## 📦 Getting Started

### 1. **Clone the Repo**
```bash
git clone https://github.com/yourusername/rust_password_generator_cli.git
cd rust_password_generator_cli
```
### 2. **Add Dependencies**

In Cargo.toml:
```toml
[dependencies]
rand = "0.8"
```
### 3. **Run the Project**
```bash
cargo run -- 20
```

🔒 This generates a 20-character password. Default is 16 if no length is provided.
