use genai::chat::{ChatMessage, ChatRequest, ChatResponse};
use genai::client::{self, Client};
use genai::utils::print_chat_stream;

#[tokio::main]
pub async fn talk_to_ai(query: &String) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();

    let chat_request = ChatRequest::new(vec![
        // vector of ChatMessages
        ChatMessage::system("Answer with at most five sentences."),
        ChatMessage::user(query),
    ]);

    // let model: &str = "gpt-3.5-turbo";
    // let model: &str = "claude-3-haiku-20240307";
    let model: &str = "gemini-1.5-flash-latest";
    // let model: &str = "llma3";
    // let model: &str = "mixtral";

    println!("\n\n --Question: {}", query);
    println!("\n--Answer from {model}.\n");

    // run this in the terminal to set your api key
    // export OPENAI_API_KEY="your_api_key"
    // export GEMINI_API_KEY="your_api_key"

    let chat_response = client.exec_chat_stream(model, chat_request, None).await?;
    print_chat_stream(chat_response, None).await?;

    Ok(())
}
