use anyhow::Result;
use heygen::{bot::HeyGenBot, settings::SETTINGS};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = SETTINGS.api_heygen_token.clone();

    let bot = HeyGenBot::new(api_key, Some("https://api.heygen.com/v1/"))?;

    let video_id = "9514f56a26864050b8d40ef7973a4859";

    match bot.retrieve_video_details(&video_id).await {
        Ok(response) => {
            if let Some(data) = response.get("data") {
                println!("Video infomation: {}", data);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(())
}
