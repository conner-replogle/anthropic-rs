use std::error::Error;

use anthropic::client::Client;
use anthropic::config::AnthropicConfig;
use anthropic::types::{Content, Message, MessagesRequestBuilder, Model, Role};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger.
    env_logger::init();

    // Load the environment variables from the .env file.
    dotenv().ok();

    // Build from configuration.
    let cfg = AnthropicConfig::new()?;
    let client = Client::try_from(cfg)?;

    let complete_request = MessagesRequestBuilder::default()
        .max_tokens(4096_u32)
        .model(Model::Claude3Haiku20240307)
        .system("Ask how the user is doing?")
        .messages(vec![Message { role: Role::User, content: Content::Text("Hello AI".to_string()) }])
        .temperature(0.5)
        .build()?;

    // Send a completion request.
    let complete_response = client.messages(complete_request).await?;

    println!("completion response: {complete_response:?}");

    Ok(())
}
