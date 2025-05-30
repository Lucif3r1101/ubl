use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufWriter, Cursor, Read, Write},
    path::Path,
};

use walkdir::WalkDir;
use zstd::stream::Encoder;

use crate::encrypt;

pub fn run(
    archive_path: &str,
    add: Option<String>,
    remove: Option<String>,
    replace: Option<String>,
    password: Option<String>,
) {
    let mut archive_file = File::open(archive_path).unwrap();
    let mut full_data = Vec::new();
    archive_file.read_to_end(&mut full_data).unwrap();

    let raw_data = if let Some(pass) = &password {
        let salt = &full_data[..16];
        let nonce = &full_data[16..28];
        let ciphertext = &full_data[28..];

        encrypt::decrypt(salt, nonce, ciphertext, pass)
    } else {
        full_data
    };

    // Step 1: Read existing entries into memory
    let mut files: HashMap<String, Vec<u8>> = HashMap::new();
    let mut reader = Cursor::new(raw_data);

    while let Ok(_) = reader.read_exact(&mut [0u8; 4]) {
        reader.set_position(reader.position() - 4);
        let mut path_len_buf = [0u8; 4];
        reader.read_exact(&mut path_len_buf).unwrap();
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

        files.insert(path, compressed_data);
    }

    // Step 2: Handle removals
    if let Some(remove_path) = &remove {
        files.remove(remove_path);
        println!("🗑 Removed: {}", remove_path);
    }

    // Step 3: Handle additions or replacements
    if let Some(add_path) = add.or(replace) {
        let add_path = Path::new(&add_path);
        if add_path.is_file() {
            insert_file(
                &mut files,
                add_path,
                add_path.file_name().unwrap().to_string_lossy().to_string(),
            );
        } else {
            for entry in WalkDir::new(add_path)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_file())
            {
                let relative = entry.path().strip_prefix(add_path).unwrap();
                let relative_path = relative.to_string_lossy().to_string();
                insert_file(&mut files, entry.path(), relative_path);
            }
        }
    }

    // Step 4: Rebuild archive
    let mut new_archive = Vec::new();
    for (path, compressed_data) in files {
        let path_bytes = path.as_bytes();
        let path_len = path_bytes.len() as u32;
        let compressed_len = compressed_data.len() as u64;
        let original_len = 0u64; // Unknown now, optional to store

        new_archive.extend(&path_len.to_le_bytes());
        new_archive.extend(path_bytes);
        new_archive.extend(&original_len.to_le_bytes());
        new_archive.extend(&compressed_len.to_le_bytes());
        new_archive.extend(&compressed_data);
    }

    let final_data = if let Some(pass) = password {
        let (salt, nonce, ciphertext) = encrypt::encrypt(&new_archive, &pass);
        [salt, nonce, ciphertext].concat()
    } else {
        new_archive
    };

    let out_file = File::create(archive_path).unwrap();
    let mut writer = BufWriter::new(out_file);
    writer.write_all(&final_data).unwrap();

    println!("✅ Archive updated.");
}

fn insert_file(files: &mut HashMap<String, Vec<u8>>, path: &Path, relative_path: String) {
    let data = fs::read(path).unwrap();
    let mut compressed = Vec::new();
    let mut encoder = Encoder::new(&mut compressed, 21).unwrap();
    encoder.write_all(&data).unwrap();
    encoder.finish().unwrap();

    files.insert(relative_path, compressed);
    println!("➕ Added/Updated: {}", path.display());
}
