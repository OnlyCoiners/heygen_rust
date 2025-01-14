use anyhow::Result;
use heygen::bot::HeyGenBot;
use serde_json::json;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = "<your-api-key>".to_string();

    let bot = HeyGenBot::new(api_key)?;

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
