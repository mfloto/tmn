mod tm_response;

use serde::Serialize;
use std::env;

#[tokio::main]
async fn main() {
    // Get environment variables
    let event_id = env::var("EVENT_ID").expect("EVENT_ID not set");
    let discord_webhook = env::var("DISCORD_WEBHOOK").expect("DISCORD_WEBHOOK not set");
    let country_code = env::var("COUNTRY_CODE").expect("COUNTRY_CODE not set");

    let current_offers = get_resale_offers(&event_id, &country_code).await;
    // Compare online offers with offers in database
    let conn = get_db_conn();
    for offer in current_offers {
        if !is_offer_in_db(&conn, &offer) {
            println!("New offer: {:?}", offer.id);
            notify_discord_server(&discord_webhook, &offer).await;
            insert_offer_into_db(&conn, &offer);
        } else {
            println!("Offer already exists: {:?}", offer.id);
        }
    }
}

//TODO: return Result
async fn get_resale_offers(resale_id: &str, country_code: &str) -> Vec<tm_response::Offer> {
    //TODO: make url configurable (e.g. for other countries)
    let res = reqwest::get(format!(
        "https://availability.ticketmaster.eu/api/v2/TM_{}/resale/{}",
        country_code, resale_id
    ))
    .await;

    let res = res.unwrap();
    if res.status() != 200 {
        println!("Error: {:?}", res.status());
        panic!("Call to Ticketmaster API failed");
    }
    let tm_res = res.json::<tm_response::TMRes>().await;

    let offers = tm_res.expect("Failed to parse json").offers;
    println!("Found {} offers \n", offers.len());
    offers
}

/// Creates a new database connection and creates the offers table if it does not exist
fn get_db_conn() -> rusqlite::Connection {
    let conn = rusqlite::Connection::open("resale.db").expect("Failed to open database");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS offers (
                  id              TEXT PRIMARY KEY,
                  price           INTEGER NOT NULL
                  )",
        (),
    )
    .expect("Failed to create table");
    conn
}

/// Checks if an offer is already in the database
fn is_offer_in_db(conn: &rusqlite::Connection, offer: &tm_response::Offer) -> bool {
    let mut stmt = conn
        .prepare("SELECT 1 FROM offers WHERE id = ?")
        .expect("Failed to prepare statement");
    stmt.exists(&[&offer.id])
        .expect("Failed to execute statement")
}

/// Inserts an offer into the database
fn insert_offer_into_db(conn: &rusqlite::Connection, offer: &tm_response::Offer) {
    conn.execute(
        "INSERT INTO offers (id, price) VALUES (?1, ?2)",
        &[&offer.id, &offer.price.total.to_string()],
    )
    .expect("Failed to insert offer");
}

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
async fn notify_discord_server(webhook: &str, offer: &tm_response::Offer) {
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
    let client = reqwest::Client::new();
    let _ = client
        .post(webhook)
        .header("Content-Type", "application/json")
        .body(discord_message_json)
        .send()
        .await
        .expect("Failed to send request");
}
