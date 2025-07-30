use std::{fs, path::PathBuf};
use std::fs::File;
use std::io::Write;

use clap::{Arg, Command};
use sha2::{Sha256, Sha512, Digest as Sha2Digest};
use sha1::Sha1;
use blake3;

#[derive(Debug)]
enum Algorithm {
    SHA256,
    SHA512,
    SHA1,
    MD5,
    BLAKE3,
}

impl Algorithm {
    fn from_str(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "sha256" => Some(Self::SHA256),
            "sha512" => Some(Self::SHA512),
            "sha1" => Some(Self::SHA1),
            "md5" => Some(Self::MD5),
            "blake3" => Some(Self::BLAKE3),
            _ => None,
        }
    }
}

fn main() {
    let matches = Command::new("rusty-hasher")
        .version("1.0")
        .author("Devaansh Pathak")
        .about("🔐 Hash your files or text quickly in Rust!")
        .arg(
            Arg::new("text")
                .long("text")
                .help("Text input to hash")
                .num_args(1)
                .conflicts_with("file"),
        )
        .arg(
            Arg::new("file")
                .long("file")
                .help("Path to file to hash")
                .num_args(1)
                .conflicts_with("text"),
        )
        .arg(
            Arg::new("algo")
                .long("algo")
                .help("Hash algorithm: sha256 (default), sha1, sha512, md5, blake3")
                .num_args(1)
                .default_value("sha256"),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .help("Path to save the hash result")
                .num_args(1),
        )
        .get_matches();

    let algo_str = matches.get_one::<String>("algo").unwrap();
    let algo = match Algorithm::from_str(algo_str) {
        Some(a) => a,
        None => {
            eprintln!("❌ Unsupported algorithm: {}", algo_str);
            std::process::exit(1);
        }
    };

    let input_data = if let Some(text) = matches.get_one::<String>("text") {
        text.as_bytes().to_vec()
    } else if let Some(file_path) = matches.get_one::<String>("file") {
        match fs::read(file_path) {
            Ok(data) => data,
            Err(e) => {
                panic!("❌ Failed to read file: {}", e);
            }
        }
    } else {
        eprintln!("❌ Please provide either --text or --file input.");
        std::process::exit(1);
    };

    let hash = match algo {
        Algorithm::SHA256 => {
            let mut hasher = Sha256::new();
            hasher.update(&input_data);
            format!("{:x}", hasher.finalize())
        }
        Algorithm::SHA512 => {
            let mut hasher = Sha512::new();
            hasher.update(&input_data);
            format!("{:x}", hasher.finalize())
        }
        Algorithm::SHA1 => {
            let mut hasher = Sha1::new();
            hasher.update(&input_data);
            format!("{:x}", hasher.finalize())
        }
        Algorithm::MD5 => {
            format!("{:x}", md5::compute(&input_data))
        }
        Algorithm::BLAKE3 => {
            let hash = blake3::hash(&input_data);
            hash.to_hex().to_string()
        }
    };

    let hash_label = match algo {
        Algorithm::SHA256 => "SHA256",
        Algorithm::SHA512 => "SHA512",
        Algorithm::SHA1 => "SHA1",
        Algorithm::MD5 => "MD5",
        Algorithm::BLAKE3 => "BLAKE3",
    };

    println!("🔐 {} hash: {}", hash_label, hash);

    if let Some(output_path) = matches.get_one::<String>("output") {
        let path = PathBuf::from(output_path);
        let mut file = File::create(path).expect("Failed to create output file");
        file.write_all(hash.as_bytes())
            .expect("Failed to write hash to file");
        println!("📁 Hash saved to: {}", output_path);
    }
}
