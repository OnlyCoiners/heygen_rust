use anyhow::Result;
use heygen::{bot::HeyGenBot, settings::SETTINGS};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = SETTINGS.api_heygen_token.clone();
    let bot = HeyGenBot::new(api_key, None)?;

    match bot.list_webhooks().await {
        Ok(webhooks) => {
            println!("Webhooks: {}", webhooks);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
