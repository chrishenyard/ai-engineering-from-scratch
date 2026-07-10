use serde::{Deserialize, Serialize};
use std::process::ExitCode;
use tokio;

#[derive(Serialize, Deserialize)]
struct Options {
    temperature: f32,
    top_p: f32,
}

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

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum StreamEvent {
    MessageStart {
        message: StreamMessage,
    },

    ContentBlockStart {
        content_block: ContentBlock,
    },

    ContentBlockDelta {
        delta: ContentDelta,
    },

    ContentBlockStop {
        #[allow(dead_code)]
        index: usize,
    },

    MessageDelta {
        delta: MessageDelta,
        usage: Usage,
    },

    MessageStop,
}

#[derive(Debug, Deserialize)]
struct StreamMessage {
    model: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ContentBlock {
    Thinking,
    Text 
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ContentDelta {
    ThinkingDelta {
        thinking: String,
    },

    TextDelta {
        text: String,
    },
}

#[derive(Debug, Deserialize, Default)]
struct Usage {
    #[serde(default)]
    input_tokens: u32,

    #[serde(default)]
    output_tokens: u32,
}

#[derive(Debug, Deserialize, Default)]
struct MessageDelta {
    #[serde(default)]
    stop_reason: Option<String>,
}

async fn post_request(
    url: &str,
    message_request: &MessageRequest,
) -> Result<(), Box<dyn std::error::Error>> {
    use futures_util::StreamExt;
    use std::io::{self, Write};

    let client = reqwest::Client::new();

    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(message_request)
        .send()
        .await?
        .error_for_status()?;

    let mut stream = res.bytes_stream();
    let mut buffer = String::new();

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        let chunk_text = std::str::from_utf8(&chunk)?;

        buffer.push_str(chunk_text);

        while let Some(newline_index) = buffer.find('\n') {
            let line = buffer[..newline_index].trim().to_string();
            buffer = buffer[newline_index + 1..].to_string();

            if line.is_empty() {
                continue;
            }

            let line = line.strip_prefix("data:").unwrap_or(&line).trim();

            if line == "[DONE]" {
                return Ok(());
            }

            let event: StreamEvent = match serde_json::from_str(line) {
                Ok(event) => event,
                Err(_) => continue,
            };

            match event {
                StreamEvent::MessageStart { message } => {
                    println!(
                        "Started response from model: {}",
                        message.model
                    );
                }

                StreamEvent::ContentBlockStart {
                    content_block,
                } => match content_block {
                    ContentBlock::Thinking { .. } => {
                        println!("\nAI Thought:");
                    }
                    ContentBlock::Text { .. } => {
                        println!("\nAI Output:");
                    }
                },

                StreamEvent::ContentBlockDelta { delta } => {
                    match delta {
                        ContentDelta::ThinkingDelta { thinking } => {
                            print!("{}", thinking);
                            io::stdout().flush()?;
                        }

                        ContentDelta::TextDelta { text } => {
                            print!("{}", text);
                            io::stdout().flush()?;
                        }
                    }
                }

                StreamEvent::ContentBlockStop { .. } => {
                    println!();
                }

                StreamEvent::MessageDelta { delta, usage } => {
                    if let Some(stop_reason) = delta.stop_reason {
                        println!("\nStop reason: {}", stop_reason);
                    }

                    println!(
                        "Tokens: input={}, output={}",
                        usage.input_tokens,
                        usage.output_tokens
                    );
                }

                StreamEvent::MessageStop => {
                    println!("\nMessage complete.");
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    let message_request = MessageRequest {
        model: "qwen3.5:0.8b".to_string(),
        max_tokens: 65536,
        stream: true,
        messages: vec![Message {
            role: "user".to_string(),
            content: "Hello, how are you?".to_string(),
        }],
        options: Options {
            temperature: 0.4,
            top_p: 0.9,
        },
    };

    let response = post_request("http://localhost:11434/v1/messages", &message_request).await;

    match response {
        Ok(_) => {
            println!();
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }

    ExitCode::SUCCESS
}