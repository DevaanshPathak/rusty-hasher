# 🔐 Rusty Hasher

A simple and fast CLI tool written in Rust to generate **SHA256** and **MD5** hashes from text or files.

![Rust](https://img.shields.io/badge/Made%20with-Rust-orange)

## ✨ Features

- 📝 Hash **plain text** or entire **files**
- 🔐 Output **SHA256** and **MD5** hashes
- 💾 Save output to `.txt` or `.json` (coming soon!)
- ⚡ Fast and lightweight

---

## 🚀 Getting Started

### 1. Clone and Build

```bash
git clone https://github.com/devaanshpathak/rusty-hasher.git
cd rusty-hasher
cargo build --release
````

### 2. Run

Hash text:

```bash
cargo run -- --text "hello world"
```

Hash file:

```bash
cargo run -- --file path/to/your/file.txt
```

Save to a file:

```bash
cargo run -- --text "hello world" --output result.txt
```

---

## 📦 Example Output

```bash
🔐 SHA256: b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
🧮   MD5: 5eb63bbbe01eeed093cb22bb8f5acdc3
📁 Hashes saved to: result.txt
```