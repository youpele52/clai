use serde::{Deserialize, Serialize};
use std::io::Error;
use std::path::Path;
use std::{fs, usize};

#[derive(Serialize, Deserialize)]
pub struct Question {
    pub query: String,
    pub id: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Questions {
    pub questions: Vec<Question>,
}

impl Questions {
    const FILE_PATH: &'static str = "question_state.json";
    pub fn load_questions() -> Result<Questions, Error> {
        let data = fs::read_to_string(Self::FILE_PATH)?;
        let questions: Questions = serde_json::from_str(&data)?;
        Ok(questions)
    }
    pub fn save_questions(questions: &Questions) -> Result<(), Error> {
        let data = serde_json::to_string(&questions)?;
        fs::write(Self::FILE_PATH, data)?;
        Ok(())
    }

    pub fn delete_questions() -> Result<(), Error> {
        let path = Path::new(Self::FILE_PATH);

        if path.exists() {
            fs::remove_file(Self::FILE_PATH)?;
            println!(
                "\n\nPrevious questions deleted successfully!\nYou can start a new chat anytime...\n"
            );
        } else {
            println!("\n\nNo previous questions found!\nYou can start a new chat anytime...\n");
        }
        Ok(())
    }
}
