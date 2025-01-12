use anyhow::Result;
use heygen::bot::HeyGenBot;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = "<your-api-key>".to_string();

    let bot = HeyGenBot::new(api_key)?;

    // This should give a response like this or similar:
    // Response: {"code":100,"data":["avatar_video.success","avatar_video.fail","avatar_video_gif.success","avatar_video_gif.fail","video_translate.success","video_translate.fail","personalized_video"],"message":null,"msg":null}
    match bot.list_webhooks_available_events().await {
        Ok(response) => {
            println!("Response: {}", response);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
