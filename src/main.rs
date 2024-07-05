use std::{env, process};
use utils::cli::{Cli, Commands};
mod utils;
use clap::{Parser, Subcommand};

fn main() {
    let cli: Cli = Cli::parse();

    match cli.command {
        Commands::Question { query, constraint } => Cli::ask_question(query, constraint),
        Commands::Delete => Cli::delete(),
        Commands::Help => Cli::help(),
    };
}
