use anyhow::Result;
use heygen::bot::HeyGenBot;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = "<your-api-key>".to_string();

    let bot = HeyGenBot::new(api_key)?;

    match bot.list_all_avatars().await {
        Ok(response) => {
            println!("response: {}", response);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
