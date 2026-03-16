![AUR Version](https://img.shields.io/aur/version/hyperfetch)
![AUR Votes](https://img.shields.io/aur/votes/hyperfetch)
![License](https://img.shields.io/badge/license-MIT-blue)
![Rust](https://img.shields.io/badge/rust-1.75+-orange)

![AUR Version](https://img.shields.io/aur/version/hyperfetch)
![AUR Votes](https://img.shields.io/aur/votes/hyperfetch)
![GitHub stars](https://img.shields.io/github/stars/revanthnemtoor/hyperfetch)
![License](https://img.shields.io/badge/license-MIT-blue)
![Rust](https://img.shields.io/badge/rust-1.75+-orange)

# hyperfetch

**hyperfetch** is an extremely fast and customizable system information tool written in Rust.

It displays system information similar to **fastfetch** or **neofetch**, but with a focus on **extreme performance, modern design, and clean output**.

---

## ✨ Features

* ⚡ **Extreme Performance** – Typical execution time in the **1–3 ms range**
* 🧠 **Hardware Caching** – Avoids repeated expensive system calls
* 🎨 **Modern Aesthetic** – Clean and visually appealing output
* ⚙️ **Configurable** – Customize behavior via `config.toml`
* 📦 **AUR Package Available** – Easy installation for Arch Linux users

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

Benchmark performed using:

```
hyperfine -N --warmup 100 "hyperfetch" "fastfetch" "neofetch" "screenfetch"
```

| Tool           | Mean Time  |
| -------------- | ---------- |
| **hyperfetch** | **1.8 ms** |
| fastfetch      | 8.3 ms     |
| neofetch       | 569 ms     |
| screenfetch    | 1.099 s    |

**Relative performance**

* **~4.6× faster than fastfetch**
* **~315× faster than neofetch**
* **~608× faster than screenfetch**

Benchmarks were run with **default configuration (logo + modules enabled)**.

---

## 📄 License

MIT License

Copyright (c) 2026 Revanth Nemtoor
