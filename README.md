# hyperfetch

`hyperfetch` is an extremely fast and customizable system information tool written in Rust. It aims to provide system details in the style of `fastfetch` or `neofetch` but with a focus on extreme performance and a premium aesthetic.

## Features

- **Extreme Performance**: Target execution time in the 1-3ms range.
- **Hardware Caching**: Efficient caching of static hardware information to avoid expensive system calls.
- **Customizable**: Configurable via `config.toml`.
- **Modern Aesthetics**: Built with a focus on visual excellence and premium design.
- **AUR Support**: Available on the Arch User Repository.

## Installation

### Arch Linux (AUR)

You can install `hyperfetch` from the AUR using an AUR helper like `yay` or `paru`:

```bash
yay -S hyperfetch
```

### From Source

Ensure you have Rust and Cargo installed, then:

```bash
git clone https://github.com/revanthnemtoor/hyperfetch.git
cd fetch
cargo build --release
```

The binary will be available at `target/release/hyperfetch`.

## Usage

Simply run:

```bash
hyperfetch
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
