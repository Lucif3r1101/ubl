use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use zstd::stream::Decoder;

pub fn run(archive_path: &str) {
    let archive_file = match File::open(archive_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open archive '{}': {}", archive_path, e);
            return;
        }
    };

    let mut reader = BufReader::new(archive_file);
    println!("Extracting from '{}'", archive_path);

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
        let _original_len = u64::from_le_bytes(original_len_buf);

        let mut compressed_len_buf = [0u8; 8];
        reader.read_exact(&mut compressed_len_buf).unwrap();
        let compressed_len = u64::from_le_bytes(compressed_len_buf);

        let mut compressed_data = vec![0u8; compressed_len as usize];
        reader.read_exact(&mut compressed_data).unwrap();

        let output_path = Path::new("output").join(&path);
        if let Some(parent) = output_path.parent() {
            create_dir_all(parent).unwrap();
        }

        let mut decoder = Decoder::new(&compressed_data[..]).unwrap();
        let mut outfile = BufWriter::new(File::create(&output_path).unwrap());
        std::io::copy(&mut decoder, &mut outfile).unwrap();

        println!("✅ Extracted: {}", output_path.display());
    }

    println!("✅ Done extracting to ./output/");
}
