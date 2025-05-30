mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};
use commands::{compress, extract, list};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Compress {
            input,
            output,
            password,
        } => compress::run(input, output, password.clone()),
        Commands::Extract { archive } => extract::run(archive),
        Commands::List { archive } => list::run(archive),
    }
}
