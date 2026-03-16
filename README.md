![AUR Version](https://img.shields.io/aur/version/hyperfetch)
![AUR Votes](https://img.shields.io/aur/votes/hyperfetch)
![License](https://img.shields.io/badge/license-MIT-blue)
![Rust](https://img.shields.io/badge/rust-1.75+-orange)

# hyperfetch
**hyperfetch** is an extremely fast and customizable system information tool written in Rust.

It displays system information similar to tools like fastfetch or neofetch, but with a focus on **extreme performance, modern design, and clean output**.

---

## ✨ Features

* ⚡ **Extreme Performance** – Target execution time in the **1–3 ms range**
* 🧠 **Hardware Caching** – Avoids repeated expensive system calls
* 🎨 **Modern Aesthetic** – Clean and visually appealing output
* ⚙️ **Configurable** – Customize via `config.toml`
* 📦 **AUR Package Available**

---

## 📦 Installation

### Arch Linux (AUR)

Install using an AUR helper:

```bash
yay -S hyperfetch
```

or manually:

```bash
git clone https://aur.archlinux.org/hyperfetch.git
cd hyperfetch
makepkg -si
```

---

### Build from Source

Ensure Rust and Cargo are installed.

```bash
git clone https://github.com/revanthnemtoor/hyperfetch.git
cd hyperfetch
cargo build --release
```

Binary location:

```
target/release/hyperfetch
```

---

## 🚀 Usage

Run:

```bash
hyperfetch
```

---

## ⚡ Performance

Example benchmark:

```
hyperfetch: ~1.6 ms
fastfetch: ~5–7 ms
neofetch: ~80–120 ms
```

Measured using `hyperfine`.

---

## 📄 License

MIT License

Copyright (c) 2026 Revanth Nemtoor
