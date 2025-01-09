use anyhow::Result;
use heygen::{bot::HeyGenBot, examples_settings::SETTINGS};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = SETTINGS.api_heygen_token.clone();

    let bot = HeyGenBot::new(api_key, Some("https://api.heygen.com/v1/"))?;

    // let video_id = "9514f56a26864050b8d40ef7973a4859";
    let video_id = "4ed7aef4959b40dd88d51757990c38c9";

    match bot.retrieve_video_details(&video_id).await {
        Ok(response) => {
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
