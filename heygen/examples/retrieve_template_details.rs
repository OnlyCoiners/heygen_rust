use anyhow::Result;
use heygen::{bot::HeyGenBot, settings::SETTINGS};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = SETTINGS.api_heygen_token.clone();

    let bot = HeyGenBot::new(api_key, Some("https://api.heygen.com/v2/"))?;

    match bot
        .retrieve_template_details("bc0756ac2b0b491f9e5d3b9f28e35f7b")
        .await
    {
        Ok(response) => {
            println!("response: {}", response);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
