use ms_teams_webhook::{send_message, send_adaptive_card};
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let webhook_url = "https://outlook.office.com/webhook/...";

    // Send a simple message
    let message = "Hello, world!";
    send_message(webhook_url, message).await?;

    // Send an adaptive card
    let title = "My Adaptive Card";
    let text = "This is the body of my adaptive card.";
    send_adaptive_card(webhook_url, title, text).await?;

    Ok(())
}
