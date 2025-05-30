use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "ubl",
    version = "0.2.0",
    author = "Lucif3r1101",
    about = "A simple Rust-based command-line tool to compress, encrypt, extract, list, verify, and update file archives using the .ubl format.",
    long_about = r#"
    UBL (Universal Backup Locker) is a versatile command-line utility designed for
    efficient and secure management of file archives. It allows you to:

    - **Compress** files and directories into a compact .ubl archive.
    - **Encrypt** archives with an optional password for enhanced security.
    - **Extract** archive contents, optionally to a specified directory.
    - **List** the files contained within an archive without extraction.
    - **Verify** the integrity of archived files to ensure data consistency.
    - **Update** existing archives by adding, removing, or replacing files.

    The .ubl format is designed to be straightforward and robust for backup
    and archival purposes.
    "#
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Compresses files or directories into a new .ubl archive.
    ///
    /// This command takes an input path (file or directory) and an output
    /// archive path. You can optionally protect the archive with a password.
    Compress {
        /// The input file or directory to compress.
        input: String,
        /// The path for the output .ubl archive.
        output: String,
        /// Optional: Password to encrypt the archive.
        #[arg(short, long)]
        password: Option<String>,
    },
    /// Extracts contents from a .ubl archive.
    ///
    /// By default, it extracts to a new directory named after the archive
    /// (without the extension). You can specify a custom output directory.
    /// If the archive is password-protected, you must provide the password.
    Extract {
        /// The path to the .ubl archive to extract.
        archive: String,
        /// Optional: Password to decrypt the archive.
        #[arg(short, long)]
        password: Option<String>,
        /// Optional: Directory where the archive contents will be extracted.
        /// If not provided, it defaults to a folder named after the archive.
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Lists the contents of a .ubl archive.
    ///
    /// This command displays the names of the files and directories stored
    /// within the specified archive without extracting them.
    /// If the archive is password-protected, the password is required.
    List {
        /// The path to the .ubl archive to list.
        archive: String,
        /// Optional: Password to decrypt the archive for listing contents.
        #[arg(short, long)]
        password: Option<String>,
    },
    /// Updates an existing .ubl archive by adding, removing, or replacing files.
    ///
    /// You must specify exactly one of `--add`, `--remove`, or `--replace`.
    /// Adding a file will append it to the archive. Removing a file will
    /// delete it from the archive. Replacing a file will update its content
    /// within the archive.
    /// If the archive is password-protected, the password is required.
    Update {
        /// The path to the .ubl archive to update.
        archive: String,
        /// Add a file or directory to the archive.
        #[arg(short, long, group = "update_action")]
        add: Option<String>,
        /// Remove a file or directory from the archive.
        #[arg(short, long, group = "update_action")]
        remove: Option<String>,
        /// Replace an existing file in the archive with a new one.
        #[arg(short, long, group = "update_action")]
        replace: Option<String>,
        /// Optional: Password to decrypt and re-encrypt the archive during update.
        #[arg(short, long)]
        password: Option<String>,
    },
    /// Verifies the integrity of a .ubl archive.
    ///
    /// This command checks if the archive is corrupted. A 'deep' verification
    /// can be performed to check the integrity of the file contents (if implemented).
    /// If the archive is password-protected, the password is required.
    Verify {
        /// The path to the .ubl archive to verify.
        archive: String,
        /// Optional: Password to decrypt the archive for verification.
        #[arg(long)]
        password: Option<String>,
        /// Perform a deep verification, checking the integrity of file contents.
        /// (Note: Actual content hashing might be a TODO item in current implementation).
        #[arg(long)]
        deep: bool,
        /// Output verification results in JSON format.
        #[arg(long)]
        json: bool,
    },
}
