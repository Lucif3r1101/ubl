use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::Instant;

use indicatif::{ProgressBar, ProgressStyle};
use walkdir::WalkDir;
use zstd::stream::Encoder;

use crate::encrypt;

pub fn run(input: &str, output: &str, password: Option<String>) {
    let input_path = Path::new(input);
    if !input_path.exists() {
        eprintln!("Input path '{}' does not exist.", input);
        return;
    }

    println!("📦 Compressing '{}' into '{}'", input, output);
    let start = Instant::now();

    let files: Vec<_> = WalkDir::new(input_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .collect();

    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}",
        )
        .unwrap()
        .progress_chars("#>-"),
    );

    let mut archive_buf = Vec::new();

    for entry in files {
        let file_path = entry.path();
        let relative_path = file_path.strip_prefix(input_path).unwrap();
        let path_str = relative_path.to_string_lossy();
        let path_bytes = path_str.as_bytes();
        let path_len = path_bytes.len() as u32;

        let data = fs::read(file_path).unwrap();
        let original_len = data.len() as u64;

        let mut compressed = Vec::new();
        let mut encoder = Encoder::new(&mut compressed, 21).unwrap();
        encoder.write_all(&data).unwrap();
        encoder.finish().unwrap();

        let compressed_len = compressed.len() as u64;

        archive_buf.extend(&path_len.to_le_bytes());
        archive_buf.extend(path_bytes);
        archive_buf.extend(&original_len.to_le_bytes());
        archive_buf.extend(&compressed_len.to_le_bytes());
        archive_buf.extend(&compressed);

        pb.set_message(path_str.to_string());
        pb.inc(1);
    }

    pb.finish_with_message("🎉 Compression done");

    let final_data = if let Some(pass) = password {
        println!("🔒 Encrypting archive...");
        let (salt, nonce, ciphertext) = encrypt::encrypt(&archive_buf, &pass);

        let mut encrypted_data = Vec::new();
        encrypted_data.extend(&salt);
        encrypted_data.extend(&nonce);
        encrypted_data.extend(&ciphertext);
        encrypted_data
    } else {
        archive_buf
    };

    let out_file = File::create(output).unwrap();
    let mut writer = BufWriter::new(out_file);
    writer.write_all(&final_data).unwrap();

    let duration = start.elapsed();
    println!("✅ Archive written to '{}'", output);
    println!("🕒 Completed in {:.2?}", duration);
}
