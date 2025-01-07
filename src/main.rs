use anyhow::Result;
use heygen_rust::{bot::HeyGenBot, settings::SETTINGS};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = SETTINGS.api_heygen_token.clone();
    let bot = HeyGenBot::new(api_key, None)?;
    // bot.get_all_avatars().await?;

    // match bot.get_all_avatars().await {
    //     Ok(avatars) => {
    //         println!("Avatars: {}", avatars);
    //     }
    //     Err(e) => eprintln!("Error: {}", e),
    // }

    match bot.list_webhooks().await {
        Ok(webhooks) => {
            println!("Webhooks: {}", webhooks);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
