use std::fs::File;
use std::io::{BufReader, Read};

pub fn run(archive_path: &str) {
    let archive_file = match File::open(archive_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open archive '{}': {}", archive_path, e);
            return;
        }
    };

    let mut reader = BufReader::new(archive_file);
    println!("📦 Listing contents of '{}'\n", archive_path);

    loop {
        let mut path_len_buf = [0u8; 4];
        if reader.read_exact(&mut path_len_buf).is_err() {
            break; // EOF
        }

        let path_len = u32::from_le_bytes(path_len_buf);
        let mut path_buf = vec![0u8; path_len as usize];
        reader.read_exact(&mut path_buf).unwrap();
        let path = String::from_utf8(path_buf).unwrap();

        let mut original_len_buf = [0u8; 8];
        reader.read_exact(&mut original_len_buf).unwrap();
        let original_len = u64::from_le_bytes(original_len_buf);

        let mut compressed_len_buf = [0u8; 8];
        reader.read_exact(&mut compressed_len_buf).unwrap();
        let compressed_len = u64::from_le_bytes(compressed_len_buf);

        // Skip compressed data
        reader
            .by_ref()
            .take(compressed_len)
            .read_to_end(&mut Vec::new())
            .unwrap();

        println!(
            "📁 {} — Original: {} bytes | Compressed: {} bytes",
            path, original_len, compressed_len
        );
    }

    println!("\n✅ Done listing.");
}
