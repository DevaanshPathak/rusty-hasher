use std::fs;
use std::fs::File;
use std::io::Write;

use clap::Parser;
use sha2::{Sha256, Digest};
use md5;

/// CLI tool to hash text or files using SHA256 and MD5
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input text to hash
    #[arg(short, long, conflicts_with = "file")]
    text: Option<String>,

    /// Path to input file
    #[arg(short, long, conflicts_with = "text")]
    file: Option<String>,

    /// Optional output file path to save the result
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();

    let input_data = if let Some(text) = args.text {
        text.into_bytes()
    } else if let Some(file_path) = args.file {
        match fs::read(&file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("❌ Failed to read file: {}", e);
                return;
            }
        }
    } else {
        eprintln!("⚠️  Please provide either --text or --file");
        return;
    };

    // SHA256
    let mut sha256 = Sha256::new();
    sha256.update(&input_data);
    let result_sha256 = sha256.finalize();
    let hash_sha256 = format!("{:x}", result_sha256);

    // MD5
    let hash_md5 = format!("{:x}", md5::compute(&input_data));

    let output_string = format!(
        "SHA256: {}\nMD5:    {}\n",
        hash_sha256, hash_md5
    );

    // Print to console
    println!("🔐 SHA256: {}", hash_sha256);
    println!("🧮   MD5: {}", hash_md5);

    // Optionally write to file
    if let Some(output_path) = args.output {
        match File::create(&output_path) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(output_string.as_bytes()) {
                    eprintln!("❌ Failed to write to file: {}", e);
                } else {
                    println!("📁 Hashes saved to: {}", output_path);
                }
            }
            Err(e) => eprintln!("❌ Failed to create file: {}", e),
        }
    }
}
