use reqwest::Client;
use serde::Serialize;

use crate::tm_response::Offer;

/// Discord webhook payload
#[derive(Debug, Serialize)]
struct DiscordMessage {
    username: String,
    embeds: Vec<DiscordEmbed>,
}

/// Discord webhook embed
#[derive(Debug, Serialize)]
struct DiscordEmbed {
    title: String,
    description: String,
    color: u32,
}

/// Sends a message via a Discord webhook
pub(crate) async fn notify_discord_server(webhook: &str, offer: &Offer) {
    // Construct Discord webhook payload
    let embed = DiscordEmbed {
        title: "New ticket available".to_string(),
        description: format!(
            "Id: {}\nPrice: {}€\nOriginal price: {}€\nRestrictions: {}",
            offer.id,
            offer.price.total / 100,
            offer.price.original / 100,
            offer.restrictions.join(", ")
        ),
        color: 0x000000,
    };
    let payload = DiscordMessage {
        username: "Ticketmaster Resale Bot".to_string(),
        embeds: vec![embed],
    };
    let discord_message_json = serde_json::to_string(&payload).unwrap();
    // Send request
    let client = Client::new();
    let _ = client
        .post(webhook)
        .header("Content-Type", "application/json")
        .body(discord_message_json)
        .send()
        .await
        .expect("Failed to send request");
}
