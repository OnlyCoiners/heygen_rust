use anyhow::Result;
use heygen::{bot::HeyGenBot, examples_settings::SETTINGS};
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
                    "input_text": "This is a example text for heygen sdk crate",
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

    match bot.create_avatar_video(payload).await {
        Ok(response) => match response.data {
            Some(data) => println!("Video generated successfully, video_id: {}", data.video_id),
            None => {
                if let Some(error) = response.error {
                    eprintln!("API error: {}, code: {}", error.message, error.code);
                }
            }
        },
        Err(e) => eprintln!("Request error: {}", e),
    }

    Ok(())
}
