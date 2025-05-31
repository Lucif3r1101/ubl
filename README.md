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

## 🚀 Installation

### 🔧 macOS (via Homebrew)

If you've installed a previous version manually, first untap it:

```bash
brew untap lucif3r1101/ubl || true
```

Then install the latest version:

```bash
brew tap lucif3r1101/ubl
brew install ubl
```

### 🐧 Linux (via `.deb` package)

```bash
wget https://github.com/Lucif3r1101/ubl/releases/download/v0.1.9/ubl-x86_64-unknown-linux-gnu.deb
sudo dpkg -i ubl-x86_64-unknown-linux-gnu.deb
ubl --help
```

> APT repository-based install is coming soon!

---

## 🛠️ Manual Build

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
ubl compress sample_dir archive.ubl
```

### With password:

```bash
ubl compress sample_dir secure.ubl --password secret123
```

---

## 📂 Extract

### Default (extracts to folder `archive/`):

```bash
ubl extract archive.ubl
```

### Custom output folder:

```bash
ubl extract archive.ubl extracted_folder
```

### Password-protected archive:

```bash
ubl extract secure.ubl --password secret123
```

Or with custom output:

```bash
ubl extract secure.ubl extracted_secure --password secret123
```

---

## 📃 List Archive Contents

### Without password:

```bash
ubl list archive.ubl
```

### With password:

```bash
ubl list secure.ubl --password secret123
```

---

## 🔍 Verify Archive Integrity

### Without password:

```bash
ubl verify archive.ubl
```

### With password:

```bash
ubl verify secure.ubl --password secret123
```

### Deep verify (checks file content if implemented):

```bash
ubl verify secure.ubl --password secret123 --deep
```

---

## 🔧 Update Archive

### Add a file (no password):

```bash
echo "Extra file" > extra.txt
ubl update archive.ubl --add extra.txt
```

### Remove a file (with password):

```bash
ubl update secure.ubl --password secret123 --remove hello.txt
```

### Replace a file:

```bash
echo "Updated README content" > readme.md
ubl update archive.ubl --replace readme.md
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
ubl compress sample_dir archive.ubl
ubl compress sample_dir secure.ubl --password secret123

# Extract
ubl extract archive.ubl
ubl extract secure.ubl --password secret123

# List
ubl list archive.ubl
ubl list secure.ubl --password secret123

# Verify
ubl verify archive.ubl
ubl verify secure.ubl --password secret123
ubl verify secure.ubl --password secret123 --deep

# Update
echo "Another file" > extra.txt
ubl update archive.ubl --add extra.txt
ubl update secure.ubl --password secret123 --remove hello.txt
ubl update archive.ubl --replace readme.md
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