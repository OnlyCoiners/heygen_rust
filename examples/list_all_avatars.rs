use anyhow::Result;
use heygen::{bot::HeyGenBot, examples_settings::SETTINGS};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = SETTINGS.api_heygen_token.clone();

    let bot = HeyGenBot::new(api_key)?;

    match bot.list_all_avatars().await {
        Ok(response) => {
            println!("response: {}", response);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
