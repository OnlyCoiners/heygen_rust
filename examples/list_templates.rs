use anyhow::Result;
use heygen::bot::HeyGenBot;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = "<your-api-key>".to_string();

    let bot = HeyGenBot::new(api_key)?;

    match bot.list_templates().await {
        Ok(response) => match response.data {
            Some(data) => {
                if data.templates.len() <= 0 {
                    println!("No templates on your account yet! Make sure that you are premium before retrieving and creating templates.")
                } else {
                    println!("Templates retrieved successfully:");
                }
                for template in data.templates {
                    println!(
                        "- Name: {}, ID: {}, Thumbnail: {}",
                        template.name, template.template_id, template.thumbnail_image_url
                    );
                }
            }
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
