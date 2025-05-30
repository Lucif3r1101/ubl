mod cli;
mod commands;
mod encrypt;

use clap::Parser;
use cli::{Cli, Commands};
use commands::{compress, extract, list, update, verify};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Compress {
            input,
            output,
            password,
        } => compress::run(input, output, password.clone()),

        Commands::Extract {
            archive,
            password,
            output,
        } => extract::run(archive, password.clone(), output.clone()),

        Commands::List { archive, password } => list::run(archive, password.clone()),

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

        Commands::Verify {
            archive,
            password,
            deep,
            json,
        } => verify::run(archive, password.clone(), *deep, *json),
    }
}
