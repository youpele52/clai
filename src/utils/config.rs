use std::error::Error;

use super::question_state::{Questions, DELETE_CHAT_PROMPT};
use super::talk_to_ai;

pub struct Config {
    pub query: String,
    pub constraint: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        // let constraint = "Answer with at most five sentences.";
        let constraint = if args.len() >= 3 {
            args[2].clone()
        } else {
            "Answer with at most five sentences.".to_string()
        };
        Ok(Config {
            query: args[1].clone(),
            constraint,
        })
    }

    pub fn query_ai(&self) -> Result<(), Box<dyn Error>> {
        if &self.query == DELETE_CHAT_PROMPT {
            Questions::delete_questions();
        } else {
            talk_to_ai(&self)?;
        }
        Ok(())
    }
}
