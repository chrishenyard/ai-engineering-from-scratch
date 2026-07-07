use serde::{Serialize, Deserialize};
use std::process::{ExitCode};
use tokio;

// Define struct for options
#[derive(Serialize, Deserialize)]
struct Options {
    temperature: f32,
    top_p: f32,
}

// Define structs for the request and response
#[derive(Serialize, Deserialize)]
struct MessageRequest {
    model: String,
    max_tokens: u32,
    stream: bool,
    messages: Vec<Message>,
    options: Options,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ContentItem {
    Thinking { thinking: String },
    Text { text: String },
}

#[derive(Deserialize, Debug, Default)]
struct Usage {
    #[serde(default)]
    input_tokens: u32,
    #[serde(default)]
    output_tokens: u32
}

#[derive(Deserialize, Debug)]
struct MessageResponse {
    id: String,
    r#type: String,
    #[serde(default)]
    role: String,
    model: String,
    #[serde(default)]
    content: Vec<ContentItem>,
    #[serde(default)]
    stop_reason: String,
    #[serde(default)]
    usage: Usage
}

// Write function for a HTTP post request to the local API server
async fn post_request(url: &str, message_request: &MessageRequest) -> Result<MessageResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client.post(url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(message_request)?)
        .send()
        .await?;

    let body = res.text().await?;
    let message_response: MessageResponse = serde_json::from_str(&body)?;
    Ok(message_response)
}

#[tokio::main]
async fn main() -> ExitCode {
    let message_request = MessageRequest {
        model: "qwen3.5:0.8b".to_string(),
        max_tokens: 65536,
        stream: false,
        messages: vec![
            Message {
                role: "user".to_string(),
                content: "Hello, how are you?".to_string(),
            }
        ],
        options: Options {
            temperature: 0.7,
            top_p: 0.9,
        },
    };

    let response = post_request("http://localhost:11434/v1/messages", &message_request).await;
    
    match response {
        Ok(res) => {
            for item in &res.content {
                    match item {
                        ContentItem::Thinking { thinking } => println!("AI Thought: {}", thinking),
                        ContentItem::Text { text } => println!("AI Output: {}", text),
                    }
                }
        },
        Err(err) => eprintln!("Error: {}", err),
    }

    ExitCode::SUCCESS
}
