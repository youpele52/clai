use std::{env, process};
use utils::config;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = config::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = config.query_ai() {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
