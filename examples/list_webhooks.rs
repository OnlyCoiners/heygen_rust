use anyhow::Result;
use heygen::bot::HeyGenBot;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = "<your-api-key>".to_string();

    let bot = HeyGenBot::new(api_key)?;

    match bot.list_webhooks().await {
        Ok(response) => {
            println!("Response: {}", response);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
