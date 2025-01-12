use anyhow::Result;
use heygen::bot::HeyGenBot;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // Save your key on api_key variable
    let api_key = "<your-api-key>".to_string();

    // Configure the bot with key
    let bot = HeyGenBot::new(api_key)?;

    // Settle your public endpoint which will receive the post request from heygen and the events for which you want to be notified
    let endpoint_url = "<your-endpoint-url>";
    let events = vec!["avatar_video.success".to_string()];

    // On the response you should see something with code 100 and data, on this data will have an endpoint_id and a secret, keep this secret, you will use once you need to work with this webhook
    match bot.register_webhook(endpoint_url, events).await {
        Ok(response) => {
            println!("Response: {}", response);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
