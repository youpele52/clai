use super::talk_to_ai;

use std::error::Error;

pub struct Config {
    pub query: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        Ok(Config {
            query: args[1].clone(),
        })
    }

    pub fn query_ai(&self) -> Result<(), Box<dyn Error>> {
        talk_to_ai(&self.query);
        Ok(())
    }
}
