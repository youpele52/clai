use clap::{Parser, Subcommand};
use std::error::Error;

use super::ai_model_state::AIModel;
use super::question_state::Questions;
use super::talk_to_ai;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]

pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(alias = "q")]
    Question {
        query: String,
        #[clap(long, short)] // can use -c or --constraint to add a constraint
        constraint: Option<String>,
    },

    #[clap(alias = "d")]
    Delete,

    #[clap(alias = "set")]
    SetAIModel { model: String },

    #[clap(alias = "h")]
    Help,
}

impl Cli {
    pub fn ask_question(query: String, constraint: Option<String>) -> Result<(), Box<dyn Error>> {
        match constraint {
            Some(constraint) => {
                talk_to_ai(query, constraint)?;
            }
            None => {
                let constraint: String = "Answer with at most five sentences.".to_string();
                talk_to_ai(query, constraint)?;
            }
        }
        Ok(())
    }

    pub fn delete() -> Result<(), Box<dyn Error>> {
        Questions::delete_questions()?;
        Ok(())
    }

    pub fn help() -> Result<(), Box<dyn Error>> {
        println!("Clai - A simple command line based program built using Rust and rust-genai.\nType 'clai -h' to see help");
        Ok(())
    }
    pub fn set_ai_model(model: String) -> Result<(), Box<dyn Error>> {
        AIModel::save_model(model)?;
        Ok(())
    }
}
