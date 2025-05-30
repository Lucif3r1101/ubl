use std::{
    fs::File,
    io::{Cursor, Read},
    path::Path,
};

use crate::encrypt;
use sha2::{Digest, Sha256};
use zstd::stream::Decoder;

use serde::Serialize;

#[derive(Serialize)]
struct FileVerificationResult {
    path: String,
    status: String,
    sha256: String,
    error: Option<String>,
}

/// Verifies archive integrity and optionally decompresses files for deep inspection.
pub fn run(archive_path: &str, password: Option<String>, deep: bool, json: bool) {
    let archive_path = Path::new(archive_path);
    if !archive_path.exists() {
        eprintln!("❌ Archive not found: {}", archive_path.display());
        std::process::exit(1);
    }

    let mut file = File::open(archive_path).unwrap_or_else(|e| {
        eprintln!("❌ Failed to open archive: {}", e);
        std::process::exit(1);
    });

    let mut full_data = Vec::new();
    file.read_to_end(&mut full_data).unwrap();

    let raw_data = {
        let result = if let Some(pass) = password {
            if full_data.len() < 28 {
                Err("❌ Archive is too small or corrupted.".to_string())
            } else {
                let salt = &full_data[..16];
                let nonce = &full_data[16..28];
                let ciphertext = &full_data[28..];

                println!("🔐 Decrypting archive...");

                let decrypted = encrypt::decrypt(salt, nonce, ciphertext, &pass);
                Ok(decrypted)
            }
        } else {
            Ok(full_data)
        };

        match result {
            Ok(data) => data,
            Err(msg) => {
                eprintln!("{}", msg);
                std::process::exit(1);
            }
        }
    };

    let mut reader = Cursor::new(raw_data);
    let mut total_files = 0;
    let mut failed = false;

    let mut results: Vec<FileVerificationResult> = Vec::new();

    while let Ok(_) = reader.read_exact(&mut [0u8; 4]) {
        reader.set_position(reader.position() - 4);

        let mut path_len_buf = [0u8; 4];
        if reader.read_exact(&mut path_len_buf).is_err() {
            break;
        }
        let path_len = u32::from_le_bytes(path_len_buf);

        let mut path_buf = vec![0u8; path_len as usize];
        reader.read_exact(&mut path_buf).unwrap();
        let path = String::from_utf8(path_buf).unwrap_or_else(|_| "<invalid utf8>".into());

        let mut _original_len = [0u8; 8];
        reader.read_exact(&mut _original_len).unwrap();

        let mut compressed_len_buf = [0u8; 8];
        reader.read_exact(&mut compressed_len_buf).unwrap();
        let compressed_len = u64::from_le_bytes(compressed_len_buf);

        let mut compressed_data = vec![0u8; compressed_len as usize];
        if reader.read_exact(&mut compressed_data).is_err() {
            results.push(FileVerificationResult {
                path,
                status: "Corrupted".into(),
                sha256: "".into(),
                error: Some("Incomplete compressed data".into()),
            });
            failed = true;
            continue;
        }

        let mut sha = Sha256::new();
        let mut status = "OK";
        let mut error_msg = None;

        if deep {
            match Decoder::new(&compressed_data[..]) {
                Ok(mut decoder) => {
                    let mut buf = Vec::new();
                    if decoder.read_to_end(&mut buf).is_err() {
                        status = "Corrupted";
                        error_msg = Some("Decompression failed".into());
                        failed = true;
                    } else {
                        sha.update(&buf);
                    }
                }
                Err(_) => {
                    status = "Corrupted";
                    error_msg = Some("Invalid zstd stream".into());
                    failed = true;
                }
            }
        } else {
            sha.update(&compressed_data);
        }

        let sha256 = format!("{:x}", sha.finalize());
        results.push(FileVerificationResult {
            path: path.clone(),
            status: status.into(),
            sha256,
            error: error_msg,
        });

        total_files += 1;
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&results).unwrap());
    } else {
        for r in &results {
            if r.status == "OK" {
                println!("✅ {:<50} — SHA-256: {}", r.path, r.sha256);
            } else {
                println!(
                    "❌ {:<50} — {}: {}",
                    r.path,
                    r.status,
                    r.error.clone().unwrap_or_default()
                );
            }
        }

        println!("\n🔍 Verified {} file(s).", total_files);
        if failed {
            println!("❗ Some files failed integrity checks.");
            std::process::exit(1);
        } else {
            println!("✅ All files passed.");
        }
    }
}
