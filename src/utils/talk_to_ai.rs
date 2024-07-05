use genai::chat::{ChatMessage, ChatRequest, ChatResponse};
use genai::client::{self, Client};
use genai::utils::print_chat_stream;

// both files are in the same directory or modules, hence we use super
use super::question_state::{Question, Questions};

#[tokio::main]
pub async fn talk_to_ai(
    query: String,
    constraint: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // load the list of questions from disk or create an empty list
    let mut question_list: Questions = Questions::load_questions().unwrap_or_else(|_| Questions {
        questions: Vec::new(),
    });

    // add a new question
    question_list.questions.push(Question {
        query: query.clone(),
        id: question_list.questions.len() + 1,
    });

    // save the list of questions
    Questions::save_questions(&question_list)?;

    let client = Client::default();

    let mut chat_request: ChatRequest = ChatRequest::default().with_system(&constraint);

    // let model: &str = "gpt-3.5-turbo";
    // let model: &str = "claude-3-haiku-20240307";
    let model: &str = "gemini-1.5-flash-latest";
    // let model: &str = "llma3";
    // let model: &str = "mixtral";

    println!("\n--Answer from {model}.\n");

    // run this in the terminal to set your api key
    // export OPENAI_API_KEY="your_api_key"
    // export GEMINI_API_KEY="your_api_key"

    for question in &question_list.questions {
        println!("\n\n--Question: {}\n", question.query);
        chat_request = chat_request.append_message(ChatMessage::user(&question.query));

        let chat_response = client
            .exec_chat_stream(model, chat_request.clone(), None)
            .await?;
        let response = print_chat_stream(chat_response, None).await?;

        chat_request = chat_request
    }

    Ok(())
}
