use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{create_dir_all, File};
use std::io::{BufRead, BufWriter, Cursor, Read};
use std::path::Path;
use std::time::Instant;
use zstd::stream::Decoder;

use crate::encrypt;

pub fn run(archive_path: &str, password: Option<String>, output: Option<String>) {
    let start = Instant::now();
    let mut file = File::open(archive_path).unwrap();
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
    println!("📦 Extracting...");

    let archive_file = Path::new(archive_path);
    let default_output_dir = archive_file
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "extracted".to_string());

    let base_output_dir = output.unwrap_or(default_output_dir);

    let mut files = vec![];

    // First pass: count entries for progress bar
    {
        let mut preview = reader.clone();
        loop {
            let mut path_len_buf = [0u8; 4];
            if preview.read_exact(&mut path_len_buf).is_err() {
                break;
            }
            let path_len = u32::from_le_bytes(path_len_buf);
            preview.consume(path_len as usize + 8 + 8); // path + original_len + compressed_len

            let mut compressed_len_buf = [0u8; 8];
            preview.read_exact(&mut compressed_len_buf).unwrap();
            let compressed_len = u64::from_le_bytes(compressed_len_buf);
            preview.consume(compressed_len as usize);

            files.push(()); // Push dummy to count
        }
    }

    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.magenta/blue}] {pos}/{len} {msg}",
        )
        .unwrap()
        .progress_chars("#>-"),
    );

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

        let output_path = Path::new(&base_output_dir).join(&path);

        if let Some(parent) = output_path.parent() {
            create_dir_all(parent).unwrap();
        }

        let mut decoder = Decoder::new(&compressed_data[..]).unwrap();
        let mut outfile = BufWriter::new(File::create(&output_path).unwrap());
        std::io::copy(&mut decoder, &mut outfile).unwrap();

        pb.set_message(path.clone());
        pb.inc(1);
    }

    pb.finish_with_message("🎉 Extraction complete");
    let duration = start.elapsed();

    println!("✅ All files restored to '{}'", base_output_dir);
    println!("🕒 Completed in {:.2?}", duration);
}
