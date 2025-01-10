use anyhow::Result;
use heygen::{bot::HeyGenBot, examples_settings::SETTINGS};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = SETTINGS.api_heygen_token.clone();

    let bot = HeyGenBot::new(api_key, Some("https://api.heygen.com/v1/"))?;

    // limit the amount of videos you want to retrieve
    let limit = 10;

    match bot.list_videos(limit).await {
        Ok(response) => match response.data {
            Some(data) => {
                if data.videos.len() <= 0 {
                    println!("No videos on your account yet!")
                } else {
                    println!("List of videos retrieved successfully:");
                }
                for video in data.videos {
                    println!(
                        "Title: {}\n ID: {}\n Status: {:?}\n Type: {:?}\n Created At: {:?}\n",
                        video.video_title,
                        video.video_id,
                        video.status,
                        video.video_type,
                        video.created_at
                    );
                }
                println!(
                    "Token for next set of videos (pagination): {}",
                    data.token.unwrap_or("No token available".to_string())
                )
            }
            None => {
                eprintln!(
                    "API error: {}, code: {}",
                    response.message.unwrap(),
                    response.code
                );
            }
        },
        Err(e) => eprintln!("Request error: {}", e),
    }

    Ok(())
}
