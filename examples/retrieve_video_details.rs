use anyhow::Result;
use heygen::{bot::HeyGenBot, examples_settings::SETTINGS};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = SETTINGS.api_heygen_token.clone();

    let bot = HeyGenBot::new(api_key)?;

    // Put your video_id
    let video_id = "7b56395626cf4ea7aa9a1b2318aec449";

    match bot.retrieve_video_details(&video_id).await {
        Ok(response) => {
            println!("response: {:?}", response);
            let video_details = response.data.unwrap();
            if let Some(video_error) = video_details.error {
                eprintln!(
                    "Video Error: Code: {}, Message: {}, Detail: {}",
                    video_error.code,
                    video_error.message,
                    video_error.detail.unwrap()
                );
            } else {
                println!("Video Details: {:#?}", video_details);
            }
        }
        Err(e) => eprintln!("Request error: {}", e),
    }
    Ok(())
}
