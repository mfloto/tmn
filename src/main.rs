mod tm_response;

use std::env;

#[derive(Debug, Clone)]
struct resale_offer {
    //TODO: add date fields (first seen, sold)
    id: String,
    price: u32,
}

#[tokio::main]
async fn main() {
    let event_id = env::var("EVENT_ID").expect("EVENT_ID not set");
    //let discord_webhook = env::var("DISCORD_WEBHOOK").expect("DISCORD_WEBHOOK not set");
    let current_offers = get_resale_offers(&event_id).await;
    for offer in current_offers {
        println!("{:?}", offer.id);
    }
}

//TODO: return Result
async fn get_resale_offers(resale_id: &str) -> Vec<tm_response::Offer> {
    //TODO: make url configurable (e.g. for other countries)
    let res = reqwest::get(format!(
        "https://availability.ticketmaster.eu/api/v2/TM_DE/resale/{}",
        resale_id
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
