mod discord;
mod tm_response;

use std::env;

use crate::discord::notify_discord_server;
use crate::tm_response::{Offer, TMRes};
use color_eyre::eyre::Context;
use color_eyre::{Result, Report, Help};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    color_eyre::install()?;
    // Get environment variables like in the first line
    let event_id = env::var("EVENT_ID").wrap_err("Unable to get EVENT_ID").suggestion("Check if EVENT_ID is correct in .env")?;
    let discord_webhook = env::var("DISCORD_WEBHOOK").wrap_err("Unable to get DISCORD_WEBHOOK").suggestion("Check if DISCORD_WEBHOOK is correct in .env")?;
    let country_code = env::var("COUNTRY_CODE").wrap_err("Unable to get COUNTRY_CODE").suggestion("Check if COUNTRY_CODE is correct in .env")?;
    let threshold_price = env::var("THRESHOLD_PRICE")
        .unwrap_or("0".to_string())
        .parse::<u32>()
        .expect("THRESHOLD_PRICE is not a number")
        * 100;

    let current_offers = get_resale_offers(&event_id, &country_code).await.wrap_err("Unable to get resale offers").suggestion("Check if Ticketmaster is up or blocked your IP")?;
    // Compare online offers with offers in database
    let conn = get_db_conn().wrap_err("Unable to get database connection").suggestion("Check if data/resale.db exists and is a valid sqlite3 database")?;
    for offer in current_offers {
        if is_offer_in_db(&conn, &offer)? {
            println!("Offer already exists: {:?}", offer.id);
        } else {
            println!("New offer: {:?}", offer.id);
            if threshold_price == 0 || offer.price.total <= threshold_price {
                println!("Price is below threshold. Sending notification...");
                notify_discord_server(&discord_webhook, &offer)
                    .await
                    .unwrap_or_else(|e| println!("Failed to send notification because of {e}"));
            }
            insert_offer_into_db(&conn, &offer).wrap_err("Unable to insert offer into database").suggestion("Check if data/resale.db exists and is writeable")?;
        }
    }
    Ok(())
}

//TODO: return Result
async fn get_resale_offers(resale_id: &str, country_code: &str) -> Result<Vec<Offer>> {
    let res = reqwest::get(format!(
        "https://availability.ticketmaster.eu/api/v2/TM_{country_code}/resale/{resale_id}"
    ))
    .await;

    let res = res?;
    if res.status() != 200 {
        println!("Error: {:?}", res.status());
        return Err(Report::msg("Could not get ticketmaster response"));
    }
    let tm_res = res.json::<TMRes>().await?;

    let offers = tm_res.offers;
    println!("Found {} offers \n", offers.len());
    Ok(offers)
}

/// Creates a new database connection and creates the offers table if it does not exist
fn get_db_conn() -> Result<rusqlite::Connection> {
    let conn = rusqlite::Connection::open("/data/resale.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS offers (
                  id              TEXT PRIMARY KEY,
                  price           INTEGER NOT NULL
                  )",
        (),
    )
    ?;
    Ok(conn)
}

/// Checks if an offer is already in the database
fn is_offer_in_db(conn: &rusqlite::Connection, offer: &Offer) -> Result<bool> {
    let mut stmt = conn
        .prepare("SELECT 1 FROM offers WHERE id = ?")?;
    Ok(stmt.exists([&offer.id])?)
}

/// Inserts an offer into the database
fn insert_offer_into_db(conn: &rusqlite::Connection, offer: &Offer) -> Result<()> {
    conn.execute(
        "INSERT INTO offers (id, price) VALUES (?1, ?2)",
        [&offer.id, &offer.price.total.to_string()],
    )?;
    Ok(())
}
