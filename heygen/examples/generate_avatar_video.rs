use anyhow::Result;
use heygen::{bot::HeyGenBot, settings::SETTINGS};
use serde_json::json;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = SETTINGS.api_heygen_token.clone();

    let bot = HeyGenBot::new(api_key, Some("https://api.heygen.com/v2/"))?;

    let payload = json!({
        "video_inputs": [
            {
                "character": {
                    "type": "avatar",
                    "avatar_id": "Angela-inTshirt-20220820",
                    "avatar_style": "normal"
                },
                "voice": {
                    "type": "text",
                    "input_text": "Welcome to the HeyGen API!",
                    "voice_id": "1bd001e7e50f421d891986aad5158bc8",
                    "speed": 1.1
                }
            }
        ],
        "dimension": {
            "width": 1280,
            "height": 720
        }
    });

    match bot.generate_avatar_video(payload).await {
        Ok(response) => {
            if let Some(data) = response.get("data") {
                println!(
                    "Video generated successfully, video_id: {}",
                    data.get("video_id").unwrap()
                );
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
