mod archive;
mod cli;
mod compressor;
mod encrypt;
mod utils;

mod commands {
    pub mod compress;
    pub mod extract;
    pub mod list;
}

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compress {
            input,
            output,
            password,
        } => {
            commands::compress::run(&input, &output, password);
        }
        Commands::Extract { archive } => {
            commands::extract::run(&archive);
        }
        Commands::List { archive } => {
            commands::list::run(&archive);
        }
    }
}
