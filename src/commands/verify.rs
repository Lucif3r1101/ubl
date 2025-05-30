use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Cursor, Read},
};

use sha2::{Digest, Sha256};
use zstd::stream::Decoder;

use crate::encrypt;

/// Verifies archive integrity and optionally checks decompressed file content.
pub fn run(archive_path: &str, password: Option<String>, deep: bool) {
    let mut file = match File::open(archive_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("❌ Failed to open archive: {}", e);
            std::process::exit(1);
        }
    };

    let mut full_data = Vec::new();
    file.read_to_end(&mut full_data).unwrap();

    let raw_data = if let Some(pass) = password {
        println!("🔐 Decrypting archive...");
        let salt = &full_data[..16];
        let nonce = &full_data[16..28];
        let ciphertext = &full_data[28..];
        encrypt::decrypt(salt, nonce, ciphertext, &pass)
    } else {
        full_data
    };

    let mut reader = Cursor::new(raw_data);
    let mut total = 0u32;
    let mut corrupted = false;

    println!("🧪 Verifying archive integrity...\n");

    while let Ok(_) = reader.read_exact(&mut [0u8; 4]) {
        reader.set_position(reader.position() - 4);

        let mut path_len_buf = [0u8; 4];
        reader.read_exact(&mut path_len_buf).unwrap();
        let path_len = u32::from_le_bytes(path_len_buf);

        let mut path_buf = vec![0u8; path_len as usize];
        reader.read_exact(&mut path_buf).unwrap();
        let path = String::from_utf8(path_buf).unwrap();

        let mut _original_len_buf = [0u8; 8];
        reader.read_exact(&mut _original_len_buf).unwrap();

        let mut compressed_len_buf = [0u8; 8];
        reader.read_exact(&mut compressed_len_buf).unwrap();
        let compressed_len = u64::from_le_bytes(compressed_len_buf);

        let mut compressed_data = vec![0u8; compressed_len as usize];
        reader.read_exact(&mut compressed_data).unwrap();

        let mut sha = Sha256::new();
        if deep {
            match Decoder::new(&compressed_data[..]) {
                Ok(mut decoder) => {
                    let mut buf = Vec::new();
                    if decoder.read_to_end(&mut buf).is_err() {
                        println!("❌ {} — Decompression failed", path);
                        corrupted = true;
                        continue;
                    }
                    sha.update(&buf);
                }
                Err(_) => {
                    println!("❌ {} — Invalid zstd stream", path);
                    corrupted = true;
                    continue;
                }
            }
        } else {
            sha.update(&compressed_data);
        }

        let hash = sha.finalize();
        println!(
            "📁 {} — SHA-256 ({}): {:x}",
            path,
            if deep { "full" } else { "compressed" },
            hash
        );

        total += 1;
    }

    println!("\n✅ Verified {} file(s).", total);
    if corrupted {
        println!("❗ One or more files failed verification.");
        std::process::exit(1);
    }
}
