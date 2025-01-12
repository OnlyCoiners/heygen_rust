use anyhow::Result;
use heygen::{
    bot::HeyGenBot,
    schemas::video::{Character, Dimension, VideoInput, VideoPayload, Voice},
};
// use serde_json::json;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // NOT WORKING RIGHT NOW
    let api_key = "<your-api-key>".to_string();

    let bot = HeyGenBot::new(api_key)?;

    // let payload = json!({
    //     "video_inputs": [
    //         {
    //             "character": {
    //                 "type": "avatar",
    //                 "avatar_id": "Angela-inTshirt-20220820",
    //                 "avatar_style": "normal"
    //             },
    //             "voice": {
    //                 "type": "text",
    //                 "input_text": "This is a example text for heygen sdk crate",
    //                 "voice_id": "1bd001e7e50f421d891986aad5158bc8",
    //                 "speed": 1.1
    //             }
    //         }
    //     ],
    //     "dimension": {
    //         "width": 1280,
    //         "height": 720
    //     }
    // });

    let video_payload = VideoPayload {
        video_inputs: vec![VideoInput {
            character: Character {
                char_type: "avatar".to_string(),
                avatar_id: "Angela-inTshirt-20220820".to_string(),
                avatar_style: "normal".to_string(),
            },
            voice: Voice {
                voice_type: "text".to_string(),
                input_text: "This is a example text for heygen sdk crate".to_string(),
                voice_id: "1bd001e7e50f421d891986aad5158bc8".to_string(),
                speed: 1.1,
            },
        }],
        dimension: Dimension::landscape(),
    };

    match bot
        .create_avatar_video_from_video_payload(video_payload)
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
