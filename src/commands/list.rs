use std::fs::File;
use std::io::{BufReader, Cursor, Read};

use crate::encrypt;

fn human_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;

    match bytes {
        b if b >= GB => format!("{:.2} GB", b as f64 / GB as f64),
        b if b >= MB => format!("{:.2} MB", b as f64 / MB as f64),
        b if b >= KB => format!("{:.2} KB", b as f64 / KB as f64),
        _ => format!("{} B", bytes), // FIXED: use `bytes` here
    }
}

pub fn run(archive_path: &str, password: Option<String>) {
    let archive_file = match File::open(archive_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("❌ Failed to open archive '{}': {}", archive_path, e);
            return;
        }
    };

    let mut full_data = Vec::new();
    let mut reader = BufReader::new(archive_file);
    if reader.read_to_end(&mut full_data).is_err() {
        eprintln!("❌ Failed to read archive.");
        return;
    }

    let raw_data = if let Some(pass) = password {
        if full_data.len() < 28 {
            eprintln!("❌ Archive is too small or corrupted.");
            return;
        }

        println!("🔐 Decrypting archive...");

        let salt = &full_data[..16];
        let nonce = &full_data[16..28];
        let ciphertext = &full_data[28..];

        encrypt::decrypt(salt, nonce, ciphertext, &pass)
    } else {
        full_data
    };

    let mut cursor = Cursor::new(raw_data);
    let mut total_original = 0u64;
    let mut total_compressed = 0u64;
    let mut file_count = 0u64;

    println!("\n📦 Contents of '{}':\n", archive_path);
    println!(
        "{:<40} {:>15} {:>15}",
        "Path", "Original Size", "Compressed Size"
    );
    println!("{:-<74}", "");

    loop {
        let mut path_len_buf = [0u8; 4];
        if cursor.read_exact(&mut path_len_buf).is_err() {
            break;
        }

        let path_len = u32::from_le_bytes(path_len_buf);
        let mut path_buf = vec![0u8; path_len as usize];
        if cursor.read_exact(&mut path_buf).is_err() {
            break;
        }

        let path = String::from_utf8(path_buf).unwrap_or_else(|_| "<invalid utf8>".into());

        let mut original_len_buf = [0u8; 8];
        if cursor.read_exact(&mut original_len_buf).is_err() {
            break;
        }
        let original_len = u64::from_le_bytes(original_len_buf);

        let mut compressed_len_buf = [0u8; 8];
        if cursor.read_exact(&mut compressed_len_buf).is_err() {
            break;
        }
        let compressed_len = u64::from_le_bytes(compressed_len_buf);

        // Skip the compressed data
        if cursor
            .by_ref()
            .take(compressed_len)
            .read_to_end(&mut Vec::new())
            .is_err()
        {
            break;
        }

        println!(
            "{:<40} {:>15} {:>15}",
            path,
            human_size(original_len),
            human_size(compressed_len)
        );

        total_original += original_len;
        total_compressed += compressed_len;
        file_count += 1;
    }

    println!("{:-<74}", "");
    println!(
        "{:<40} {:>15} {:>15}",
        "TOTAL",
        human_size(total_original),
        human_size(total_compressed)
    );
    println!("\n📄 {} files listed.\n✅ Done.", file_count);
}
