use anyhow::Result;
use heygen::bot::HeyGenBot;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // Save your key on api_key variable
    let api_key = "<your-api-key>".to_string();

    // Configure the bot with key
    let bot = HeyGenBot::new(api_key)?;

    // List webhooks if you don't know the endpoint_id and the url of the webhook you want to modify.
    // Check list_webhooks.rs example to do this
    let endpoint_id = "e51d50aa72d8466a94d4456341f23efa";
    let new_url = "https://www.google.com/";
    let new_events = vec![
        "avatar_video.success".to_string(),
        "avatar_video.fail".to_string(),
        "avatar_video_gif.success".to_string(),
    ];

    // After running this command you will get the new webhook as response with the updated informations
    match bot.update_webhook(&endpoint_id, &new_url, new_events).await {
        Ok(response) => {
            println!("Response: {}", response);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
