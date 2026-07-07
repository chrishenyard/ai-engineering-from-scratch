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
    messages: Vec<Message>,
    options: Options,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

// Write function for a HTTP post request to the local API server
async fn post_request(url: &str, message_request: &MessageRequest) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let res = client.post(url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(message_request)?)
        .send()?;

    let response_text = res.text()?;
    Ok(response_text)
}

#[tokio::main]
async fn main() -> ExitCode {
    let message_request = MessageRequest {
        model: "qwen3.5:0.8b".to_string(),
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
        Ok(res) => println!("Response: {}", res),
        Err(err) => eprintln!("Error: {}", err),
    }

    ExitCode::SUCCESS
}
