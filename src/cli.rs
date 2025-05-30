use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ubl", version = "0.1.0", author = "You")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Compress {
        input: String,
        output: String,
        #[arg(short, long)]
        password: Option<String>,
    },
    Extract {
        archive: String,
        #[arg(short, long)]
        password: Option<String>,
    },
    List {
        archive: String,
    },
}
