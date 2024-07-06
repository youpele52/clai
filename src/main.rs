use utils::cli::{Cli, Commands};
mod utils;
use clap::Parser;

fn main() {
    let cli: Cli = Cli::parse();

    let _ = match cli.command {
        Commands::Question { query, constraint } => Cli::ask_question(query, constraint),
        Commands::Delete => Cli::delete(),
        Commands::Help => Cli::help(),
        Commands::SetAIModel { model } => Cli::set_ai_model(model),
    };
}
