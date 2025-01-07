use anyhow::Result;
use heygen::{bot::HeyGenBot, settings::SETTINGS};
use serde_json::json;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = SETTINGS.api_heygen_token.clone();

    let bot = HeyGenBot::new(api_key, Some("https://api.heygen.com/v2/"))?;

    // Put your template_id here (this is mine and won't work with your key)
    let template_id = "bc0756ac2b0b491f9e5d3b9f28e35f7b";

    // Variables coming from my template, you need to adjust to yours
    let payload = json!({
        "test": true,
        "caption": false,
        "template_id": "bc0756ac2b0b491f9e5d3b9f28e35f7b",
        "title":"Video generated from template SDK",
        "variables": {
            "script": {
                "name": "script",
                "type": "text",
                "properties": {
                    "content": "Welcome to the HeyGen rust sdk powered by onlycoiners!"
                }
            },
        },
        "dimension": {
            "width": 720,
            "height": 1280
        }
    });

    match bot
        .generate_video_from_template(&template_id, payload)
        .await
    {
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
