use anyhow::Result;
use heygen::{bot::HeyGenBot, examples_settings::SETTINGS};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = SETTINGS.api_heygen_token.clone();

    let bot = HeyGenBot::new(api_key)?;

    // let template = "bc0756ac2b0b491f9e5d3b9f28e35f7b";
    let template = "44f4ae77168d45cfbc6dcbb87ea03b25";

    match bot.retrieve_template_details(&template).await {
        Ok(response) => match response.data {
            Some(data) => println!("Template Details: {:#?}", data),
            None => {
                if let Some(error) = response.error {
                    eprintln!("API error: {}, code: {}", error.message, error.code);
                }
            }
        },
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
