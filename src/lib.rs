extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Debug)]
struct Message {
    text: String,
}

#[derive(Serialize, Debug)]
struct Card {
    title: String,
    text: String,
}

#[derive(Serialize, Debug)]
struct AdaptiveCard {
    type: String,
    body: Vec<Card>,
}

#[derive(Serialize, Debug)]
struct TeamsMessage {
    #[serde(rename = "@type")]
    type: String,
    text: String,
    summary: String,
    sections: Vec<AdaptiveCard>,
}

pub async fn send_message(webhook_url: &str, message: &str) -> Result<(), reqwest::Error> {
    let message = Message { text: message.to_string() };

    let client = reqwest::Client::new();
    let res = client
        .post(webhook_url)
        .json(&message)
        .send()
        .await?;

    println!("Status: {}", res.status());

    Ok(())
}

pub async fn send_adaptive_card(
    webhook_url: &str,
    title: &str,
    text: &str,
) -> Result<(), reqwest::Error> {
    let card = Card {
        title: title.to_string(),
        text: text.to_string(),
    };

    let adaptive_card = AdaptiveCard {
        type: "AdaptiveCard".to_string(),
        body: vec![card],
    };

    let teams_message = TeamsMessage {
        type: "MessageCard".to_string(),
        text: "Adaptive Card".to_string(),
        summary: "Adaptive Card Summary".to_string(),
        sections: vec![adaptive_card],
    };

    let client = reqwest::Client::new();
    let res = client
        .post(webhook_url)
        .json(&teams_message)
        .send()
        .await?;

    println!("Status: {}", res.status());

    Ok(())
}
