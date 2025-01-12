use anyhow::Result;
use heygen::bot::HeyGenBot;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // Save your key on api_key variable
    let api_key = "<your-api-key>".to_string();

    // Configure the bot with key
    let bot = HeyGenBot::new(api_key)?;

    // List webhooks if you don't know the endpoint_id of the url you want to delete.
    // Check list_webhooks.rs example to do this
    let endpoint_id = "33af8a723c654dd381e62a8650da3890";

    // Once you run this command, it will delete the webhook with the specified endpoint_id, their server response is very strange so if you see this response:
    // Response: {"code":100,"data":null,"message":null,"msg":null}
    // It means that the webhook has been deleted, do a list.webhooks to verify
    match bot.delete_webhook(&endpoint_id).await {
        Ok(response) => {
            println!("Response: {}", response);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
