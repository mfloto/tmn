mod tm_response;

use std::env;

#[tokio::main]
async fn main() {
    let event_id = env::var("EVENT_ID").expect("EVENT_ID not set");
    //let discord_webhook = env::var("DISCORD_WEBHOOK").expect("DISCORD_WEBHOOK not set");
    get_resale_offers(&event_id).await;
}

async fn get_resale_offers(resale_id: &str) {
    let res = reqwest::get(format!(
        "https://availability.ticketmaster.eu/api/v2/TM_DE/resale/{}",
        resale_id
    ))
    .await;
    if res.is_err() {
        println!("Error: {:?}", res.err());
        return;
    }
    let res = res.unwrap();
    if res.status() != 200 {
        println!("Error: {:?}", res.status());
        return;
    }
    let tm_res = &res.json::<tm_response::TMRes>().await;
    if tm_res.as_ref().is_err() {
        println!("Error: {:?}", tm_res.as_ref().err());
        return;
    }
    let offers = &tm_res.as_ref().unwrap().offers;
    println!("Found {} offers \n", offers.len());
    for offer in offers {
        println!(
            "id: {id}\nprice: {price} €\noriginal price: {org_price} €\n",
            id = offer.id,
            price = offer.price.total / 100,
            org_price = offer.price.original / 100
        );
    }
}
