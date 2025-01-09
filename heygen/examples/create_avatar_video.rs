use anyhow::Result;
use heygen::{bot::HeyGenBot, examples_settings::SETTINGS};
use serde_json::json;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = SETTINGS.api_heygen_token.clone();

    let bot = HeyGenBot::new(api_key, Some("https://api.heygen.com/v2/"))?;

    let text = " Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam a lacus lobortis, sodales lacus in, efficitur orci. Nulla dolor diam, dapibus in enim eu, rhoncus rhoncus erat. Morbi quis libero vel sapien mattis ultrices. Curabitur augue ipsum, scelerisque eu quam ac, ornare pulvinar enim. Morbi porttitor hendrerit elit, sed porta ligula consectetur et. Pellentesque et euismod sapien. Aliquam tristique neque eleifend mauris congue, a vestibulum ligula gravida. Fusce augue elit, blandit non metus ac, ultricies condimentum nisl.

Mauris euismod nisl id dapibus suscipit. Cras pretium lorem at libero ultrices iaculis. Fusce lorem tortor, porttitor a lobortis placerat, scelerisque non arcu. Quisque molestie quam eu ligula blandit, ac faucibus ligula maximus. Maecenas a sapien erat. Phasellus quis nisi sagittis enim ultricies condimentum eu sit amet turpis. Sed dignissim sem metus, vitae laoreet orci tincidunt eu. Maecenas eu mauris lacinia, tempus nunc id, semper magna. Pellentesque sodales elit eros, ac tempus diam pellentesque interdum. Donec id mi nisl.

Aliquam erat volutpat. Donec placerat ipsum vel cursus accumsan. Quisque non orci vitae sapien euismod bibendum ut vitae nisi. Nulla consequat ex nec magna aliquam viverra. Nullam ac nibh auctor arcu vehicula ornare in id eros. Praesent ac eleifend est, tempor sagittis sapien. Nulla congue ac eros sit amet rutrum. Nam dolor metus, ornare vitae dapibus vel, porta eget quam. Proin sed turpis sit amet est hendrerit gravida.

Nullam accumsan auctor lorem vitae consectetur. Aliquam tortor dolor, congue in efficitur eu, laoreet non arcu. Morbi lacinia urna vitae dui efficitur mattis. Sed vel odio ac mauris molestie sollicitudin eu vitae ligula. Nam sit amet lorem id tellus euismod sagittis in id purus. In enim elit, suscipit sed vehicula sodales, dictum vel ex. Praesent congue molestie consectetur. In posuere nibh nunc, ut porttitor elit consequat in. Quisque posuere, eros ullamcorper semper scelerisque, nunc odio finibus sem, sit amet lacinia urna turpis vel dolor. Cras mauris justo, egestas sit amet nulla ut, dignissim fringilla nunc. Donec dignissim venenatis libero a fermentum. Cras est eros, bibendum a ex nec, consectetur feugiat dolor. Vestibulum mi dui, interdum id egestas sit amet, condimentum sit amet ex. Phasellus turpis erat, accumsan et posuere quis, porta non enim.

Donec semper tortor sed justo mattis, ac tempor nisl pulvinar. In molestie gravida nunc eu placerat. Nunc at dapibus libero. Sed pharetra non sapien ut maximus. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Proin lacinia posuere gravida. Aenean ligula lectus, tincidunt et velit at, sodales lobortis dui. Maecenas eu nisl velit. Vivamus hendrerit nisl eu enim ullamcorper eleifend. Suspendisse viverra, magna vel rhoncus placerat, justo tellus vehicula turpis, eu viverra massa massa vel felis. Fusce arcu felis, porta nec viverra ac, rhoncus quis quam. Praesent vel nulla vitae velit ornare elementum. Fusce hendrerit urna nec metus fermentum vehicula.

Vivamus condimentum aliquet luctus. Donec pellentesque, odio sit amet pharetra euismod, est nunc tincidunt mi, blandit bibendum lectus lacus a diam. Vivamus feugiat maximus bibendum. Sed id risus vel nibh maximus rhoncus a vitae ex. Quisque justo lorem, vulputate ut dolor id, consectetur vulputate mauris. Pellentesque ornare nibh et lorem finibus mollis. Aliquam erat volutpat. Suspendisse potenti. Morbi imperdiet magna vel eros iaculis hendrerit sed sit amet diam. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nam a facilisis tortor, non rhoncus mi. Mauris commodo enim at lectus maximus, in tempor lacus gravida.







Mauris id augue fringilla, pharetra eros in, pellentesque nibh. Fusce elementum a velit vitae fermentum. Nulla tempus fringilla mi, sit amet cursus est accumsan sed. Pellentesque enim nibh, pharetra consequat maximus id, ultricies in diam. Vivamus in sollicitudin lectus, eu condimentum leo. Proin quis lorem eu ipsum rhoncus semper non eu mauris. Nullam cursus elementum tellus sed facilisis. Cras vitae ex nec urna blandit fringilla quis nec nibh. Nam nulla dolor, cursus vitae ullamcorper ac, sodales nec ante. Mauris nisl tortor, tristique sit amet mi eu, eleifend fringilla lectus. Nulla arcu leo, porta eget nisl non, suscipit rutrum lorem. Donec nec scelerisque odio. ";

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
                    "input_text": text,
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
