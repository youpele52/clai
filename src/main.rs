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
    // let args: Vec<String> = env::args().collect();

    // let config = config::Config::new(&args).unwrap_or_else(|err| {
    //     println!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });

    // if let Err(e) = config.query_ai() {
    //     println!("Application error: {}", e);
    //     process::exit(1);
    // }
}
