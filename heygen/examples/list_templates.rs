use anyhow::Result;
use heygen::{bot::HeyGenBot, schemas::template::ListTemplatesResponse, settings::SETTINGS};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = SETTINGS.api_heygen_token.clone();

    let bot = HeyGenBot::new(api_key, Some("https://api.heygen.com/v2/"))?;

    let response = bot.list_templates().await;

    match response {
        Ok(ListTemplatesResponse {
            data: Some(template_list_data),
            error: None,
        }) => {
            println!(
                "Template list retrieved sucessfully: {:?}",
                template_list_data
            );
        }
        Ok(ListTemplatesResponse {
            data: None,
            error: Some(error_data),
        }) => {
            eprintln!(
                "Failed to list templates. Error code: {}, message: {}",
                error_data.code, error_data.message
            );
        }
        Ok(_) => {
            // Unexpected case: Handle it gracefully
            eprintln!("Unexpected response format received.");
        }
        Err(e) => {
            // Handle any other errors (e.g., network issues, deserialization errors)
            eprintln!("Error occurred while listing templates: {}", e);
        }
    }

    Ok(())
}
