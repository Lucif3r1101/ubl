# 🛡️ UBL - Universal Backup Locker

A simple Rust-based command-line tool to compress, encrypt, extract, list, verify, and update file archives using the `.ubl` format.

---

## 📦 Features

- Compress files/folders into a `.ubl` archive
- Optional password encryption
- Extract with automatic folder naming
- List archive contents
- Verify integrity of archived files
- Update existing archives (add/remove/replace files)

---

## 🚀 Getting Started

### 1. Build the Project

```bash
cargo build --release
```

This creates the binary at:  
```bash
./target/release/ubl
```

---

## 📁 Example Setup

```bash
mkdir sample_dir
echo "Hello, World!" > sample_dir/hello.txt
echo "This is a test" > sample_dir/readme.md
```

---

## 🗜️ Compress

### Without password:

```bash
./target/release/ubl compress sample_dir archive.ubl
```

### With password:

```bash
./target/release/ubl compress sample_dir secure.ubl --password secret123
```

---

## 📂 Extract

### Default (extracts to folder `archive/`):

```bash
./target/release/ubl extract archive.ubl
```

### Custom output folder:

```bash
./target/release/ubl extract archive.ubl extracted_folder
```

### Password-protected archive:

```bash
./target/release/ubl extract secure.ubl --password secret123
```

Or with custom output:

```bash
./target/release/ubl extract secure.ubl extracted_secure --password secret123
```

---

## 📃 List Archive Contents

### Without password:

```bash
./target/release/ubl list archive.ubl
```

### With password:

```bash
./target/release/ubl list secure.ubl --password secret123
```

---

## 🔍 Verify Archive Integrity

### Without password:

```bash
./target/release/ubl verify archive.ubl
```

### With password:

```bash
./target/release/ubl verify secure.ubl --password secret123
```

### Deep verify (checks file content if implemented):

```bash
./target/release/ubl verify secure.ubl --password secret123 --deep
```

---

## 🔧 Update Archive

### Add a file (no password):

```bash
echo "Extra file" > extra.txt
./target/release/ubl update archive.ubl --add extra.txt
```

### Remove a file (with password):

```bash
./target/release/ubl update secure.ubl --password secret123 --remove hello.txt
```

### Replace a file:

```bash
echo "Updated README content" > readme.md
./target/release/ubl update archive.ubl --replace readme.md
```

---

## 📝 Summary of Commands

| Action             | Command Example |
|--------------------|-----------------|
| Compress           | `ubl compress input_dir output.ubl [--password xxx]` |
| Extract            | `ubl extract archive.ubl [output_dir] [--password xxx]` |
| List               | `ubl list archive.ubl [--password xxx]` |
| Verify             | `ubl verify archive.ubl [--password xxx] [--deep]` |
| Update - Add       | `ubl update archive.ubl --add file.txt [--password xxx]` |
| Update - Remove    | `ubl update archive.ubl --remove file.txt [--password xxx]` |
| Update - Replace   | `ubl update archive.ubl --replace file.txt [--password xxx]` |

---

## 🧪 Testing All Functionalities

```bash
# Compress
./target/release/ubl compress sample_dir archive.ubl
./target/release/ubl compress sample_dir secure.ubl --password secret123

# Extract
./target/release/ubl extract archive.ubl
./target/release/ubl extract secure.ubl --password secret123

# List
./target/release/ubl list archive.ubl
./target/release/ubl list secure.ubl --password secret123

# Verify
./target/release/ubl verify archive.ubl
./target/release/ubl verify secure.ubl --password secret123
./target/release/ubl verify secure.ubl --password secret123 --deep

# Update
echo "Another file" > extra.txt
./target/release/ubl update archive.ubl --add extra.txt
./target/release/ubl update secure.ubl --password secret123 --remove hello.txt
./target/release/ubl update archive.ubl --replace readme.md
```

---

## 🔐 Password Encryption

- Archives encrypted with a password will **require the password** to extract, list, verify, or update.
- Without the correct password, the tool will error or output corrupted data.

---

## 🧹 TODO / Enhancements

- [ ] Optional compression levels
- [ ] Archive metadata info
- [ ] Deep verify hashing (SHA-256)
- [ ] Recursive update support

---

## 📣 Author

Made with ❤️ using Rust.
