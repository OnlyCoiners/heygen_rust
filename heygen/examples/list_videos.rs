use anyhow::Result;
use heygen::{bot::HeyGenBot, settings::SETTINGS};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = SETTINGS.api_heygen_token.clone();

    let bot = HeyGenBot::new(api_key, Some("https://api.heygen.com/v1/"))?;

    // limit the amount of videos you want to retrieve
    let limit = 10;

    match bot.list_videos(limit).await {
        Ok(response) => {
            println!("response: {}", response);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
