
# 🔐 Rusty Hasher

A sleek, fast CLI tool written in **Rust** to generate secure cryptographic hashes from **text** or **files** with style ✨.

![Rust](https://img.shields.io/badge/Made%20with-Rust-orange)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-blue)
---

## ✨ Features

- 🔤 Hash **text input** or entire **files**
- 🔐 Supports multiple algorithms:
  - `sha256`
  - `md5`
  - `sha1`
  - `sha512`
  - `blake3`
- 💾 Save output to `.txt` or `.json`
- 📋 Copy hash to clipboard with `--copy`
- ⏱️ Measure performance with `--benchmark`
- 🎨 Colorful terminal output
- 🪶 Blazingly lightweight and cross-platform

---

## 🚀 Getting Started

### 1. Clone and Build

```bash
git clone https://github.com/devaanshpathak/rusty-hasher.git
cd rusty-hasher
cargo build --release
````

### 2. Run It

#### Hash plain text

```bash
cargo run -- --text "hello world"
```

#### Hash a file

```bash
cargo run -- --file ./example.txt
```

#### Specify algorithm

```bash
cargo run -- --text "secure" --algo sha512
cargo run -- --file ./data.bin --algo blake3
```

#### Save output to a file

```bash
cargo run -- --text "hash me" --output result.txt
```

#### Copy to clipboard

```bash
cargo run -- --text "copied hash" --copy
```

#### Benchmark hashing speed

```bash
cargo run -- --file ./large.iso --benchmark
```

---

## 📦 Example Output

```bash
🔐 SHA512 hash: d2a3f8cbb36495d17c02... [truncated]
📁 Hash saved to: result.txt
📋 Copied to clipboard!
⏱️  Hashed in 3.21 ms
```
---

## 🛠️ Dependencies

* [clap](https://docs.rs/clap/) for argument parsing
* [sha2, md5, sha1, blake3](https://crates.io/) for hashing
* [colored](https://crates.io/crates/colored) for terminal styling
* [arboard](https://crates.io/crates/arboard) for clipboard support

---
> Built with ❤️ by [Devaansh Pathak](https://github.com/devaanshpathak)
