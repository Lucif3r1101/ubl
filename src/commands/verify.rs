use std::{
    fs::File,
    io::{BufReader, Cursor, Read},
};

use sha2::{Digest, Sha256};
use zstd::stream::Decoder;

use crate::encrypt;

pub fn run(archive_path: &str, password: Option<String>, deep: bool) {
    let mut file = File::open(archive_path).expect("❌ Failed to open archive.");
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
    println!("🧪 Verifying archive integrity...\n");

    let mut file_count = 0;

    loop {
        let mut path_len_buf = [0u8; 4];
        if reader.read_exact(&mut path_len_buf).is_err() {
            break;
        }

        let path_len = u32::from_le_bytes(path_len_buf);
        let mut path_buf = vec![0u8; path_len as usize];
        reader.read_exact(&mut path_buf).unwrap();
        let path = String::from_utf8(path_buf).unwrap();

        let mut original_len_buf = [0u8; 8];
        reader.read_exact(&mut original_len_buf).unwrap();

        let mut compressed_len_buf = [0u8; 8];
        reader.read_exact(&mut compressed_len_buf).unwrap();
        let compressed_len = u64::from_le_bytes(compressed_len_buf);

        let mut compressed_data = vec![0u8; compressed_len as usize];
        reader.read_exact(&mut compressed_data).unwrap();

        let hash = if deep {
            let mut decoder = Decoder::new(&compressed_data[..]).unwrap();
            let mut decompressed_data = Vec::new();
            std::io::copy(&mut decoder, &mut decompressed_data).unwrap();

            let mut hasher = Sha256::new();
            hasher.update(&decompressed_data);
            hasher.finalize()
        } else {
            let mut hasher = Sha256::new();
            hasher.update(&compressed_data);
            hasher.finalize()
        };

        println!(
            "📁 {} — SHA-256 ({}): {:x}",
            path,
            if deep { "full" } else { "compressed" },
            hash
        );

        file_count += 1;
    }

    println!("\n✅ Verified {} file(s).", file_count);
}
