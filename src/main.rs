mod cli;
mod commands;
mod encrypt;

use clap::Parser;
use cli::{Cli, Commands};
use commands::{compress, extract, list, update};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Compress {
            input,
            output,
            password,
        } => compress::run(input, output, password.clone()),

        Commands::Extract { archive, password } => extract::run(archive, password.clone()),

        Commands::List { archive } => list::run(archive),

        Commands::Update {
            archive,
            add,
            remove,
            replace,
            password,
        } => update::run(
            archive,
            add.clone(),
            remove.clone(),
            replace.clone(),
            password.clone(),
        ),
    }
}
