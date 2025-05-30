use std::fs::{self, File};
use std::io::{BufWriter, Write};
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

    let archive_file = match File::create(output) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create output file: {}", e);
            return;
        }
    };

    let mut writer = BufWriter::new(archive_file);

    for entry in WalkDir::new(input_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let file_path = entry.path();
        let relative_path = file_path.strip_prefix(input_path).unwrap();

        let data = match fs::read(file_path) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Failed to read file {}: {}", file_path.display(), e);
                continue;
            }
        };

        // Write header (path + size)
        let path_str = relative_path.to_string_lossy();
        let path_bytes = path_str.as_bytes();
        let path_len = path_bytes.len() as u32;
        let data_len = data.len() as u64;

        // Header = [path_len (4 bytes)][path][data_len (8 bytes)][compressed_data]
        writer.write_all(&path_len.to_le_bytes()).unwrap();
        writer.write_all(path_bytes).unwrap();
        writer.write_all(&data_len.to_le_bytes()).unwrap();

        // Compress the data using zstd
        let mut encoder = Encoder::new(&mut writer, 21).unwrap(); // 0–21 compression level
        encoder.write_all(&data).unwrap();
        encoder.finish().unwrap();
    }

    println!("Compression complete ✅");
}
