mod discord;
mod tm_response;

use std::env;

use crate::discord::notify_discord_server;
use crate::tm_response::{Offer, TMRes};

#[tokio::main]
async fn main() {
    // Get environment variables
    let event_id = env::var("EVENT_ID").expect("EVENT_ID not set");
    let discord_webhook = env::var("DISCORD_WEBHOOK").expect("DISCORD_WEBHOOK not set");
    let country_code = env::var("COUNTRY_CODE").expect("COUNTRY_CODE not set");
    let threshold_price = env::var("THRESHOLD_PRICE")
        .unwrap_or("0".to_string())
        .parse::<u32>()
        .expect("THRESHOLD_PRICE is not a number")
        * 100;

    let current_offers = get_resale_offers(&event_id, &country_code).await;
    // Compare online offers with offers in database
    let conn = get_db_conn();
    for offer in current_offers {
        if is_offer_in_db(&conn, &offer) {
            println!("Offer already exists: {:?}", offer.id);
        } else {
            println!("New offer: {:?}", offer.id);
            if threshold_price == 0 || offer.price.total <= threshold_price {
                println!("Price is below threshold. Sending notification...");
                notify_discord_server(&discord_webhook, &offer).await;
            }
            insert_offer_into_db(&conn, &offer);
        }
    }
}

//TODO: return Result
async fn get_resale_offers(resale_id: &str, country_code: &str) -> Vec<Offer> {
    //TODO: make url configurable (e.g. for other countries)
    let res = reqwest::get(format!(
        "https://availability.ticketmaster.eu/api/v2/TM_{country_code}/resale/{resale_id}"
    ))
    .await;

    let res = res.unwrap();
    if res.status() != 200 {
        println!("Error: {:?}", res.status());
        panic!("Call to Ticketmaster API failed");
    }
    let tm_res = res.json::<TMRes>().await;

    let offers = tm_res.expect("Failed to parse json").offers;
    println!("Found {} offers \n", offers.len());
    offers
}

/// Creates a new database connection and creates the offers table if it does not exist
fn get_db_conn() -> rusqlite::Connection {
    let conn = rusqlite::Connection::open("/data/resale.db").expect("Failed to open database");
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
fn is_offer_in_db(conn: &rusqlite::Connection, offer: &Offer) -> bool {
    let mut stmt = conn
        .prepare("SELECT 1 FROM offers WHERE id = ?")
        .expect("Failed to prepare statement");
    stmt.exists([&offer.id])
        .expect("Failed to execute statement")
}

/// Inserts an offer into the database
fn insert_offer_into_db(conn: &rusqlite::Connection, offer: &Offer) {
    conn.execute(
        "INSERT INTO offers (id, price) VALUES (?1, ?2)",
        [&offer.id, &offer.price.total.to_string()],
    )
    .expect("Failed to insert offer");
}
