use std::fs::{self, File};
use std::io::{BufWriter, Cursor, Write};
use std::path::Path;

use walkdir::WalkDir;
use zstd::stream::Encoder;

pub fn run(input: &str, output: &str, _password: Option<String>) {
    let input_path = Path::new(input);
    if !input_path.exists() {
        eprintln!("Input path '{}' does not exist.", input);
        return;
    }

    println!("Compressing '{}' into '{}'", input, output);

    let archive_file = File::create(output).expect("Failed to create archive");
    let mut writer = BufWriter::new(archive_file);

    for entry in WalkDir::new(input_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let file_path = entry.path();
        let relative_path = file_path.strip_prefix(input_path).unwrap();
        let path_str = relative_path.to_string_lossy();
        let path_bytes = path_str.as_bytes();
        let path_len = path_bytes.len() as u32;

        let data = fs::read(file_path).expect("Failed to read input file");
        let original_len = data.len() as u64;

        // Compress into memory first
        let mut compressed_buf = Vec::new();
        let mut encoder = Encoder::new(&mut compressed_buf, 21).unwrap();
        encoder.write_all(&data).unwrap();
        encoder.finish().unwrap();

        let compressed_len = compressed_buf.len() as u64;

        // Write header
        writer.write_all(&path_len.to_le_bytes()).unwrap();
        writer.write_all(path_bytes).unwrap();
        writer.write_all(&original_len.to_le_bytes()).unwrap();
        writer.write_all(&compressed_len.to_le_bytes()).unwrap();

        // Write compressed data
        writer.write_all(&compressed_buf).unwrap();

        println!(
            "📦 Added: {} ({} → {})",
            path_str, original_len, compressed_len
        );
    }

    println!("✅ Compression complete");
}
