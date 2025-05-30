use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ubl", version, about = "Ultra compression with .ubl")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Compress folder/file into .ubl
    Compress {
        input: String,
        output: String,
        #[arg(short, long)]
        password: Option<String>,
    },

    /// Extract archive
    Extract { archive: String },

    /// List contents of .ubl
    List { archive: String },
}
