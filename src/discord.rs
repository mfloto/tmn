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
        title: format!("New offer for {}", offer.id),
        description: format!("Price: {}", offer.price.total),
        color: 0x00ff00,
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
