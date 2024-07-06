use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Error;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct AIModel {
    pub model: String,
}

impl AIModel {
    const FILE_PATH: &'static str = "ai_model_state.json";
    pub fn load_model() -> Result<AIModel, Error> {
        let data = fs::read_to_string(Self::FILE_PATH)?;
        let ai_model: AIModel = serde_json::from_str(&data)?;
        Ok(ai_model)
    }
    pub fn save_model(model: String) -> Result<(), Error> {
        let model: AIModel = AIModel { model: model };
        println!("\nSaving AI model...");
        let data = serde_json::to_string(&model)?;
        fs::write(Self::FILE_PATH, data)?;
        println!("Successfully saved AI model.\n");
        Ok(())
    }
}
