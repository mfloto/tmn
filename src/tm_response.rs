use serde::Deserialize;

///Holds information about a ticketmaster event
#[derive(Debug, Deserialize)]
pub struct tm_res {
    pub offers: Vec<offer>,
}

///Holds information about a resale offer
#[derive(Debug, Deserialize)]
pub struct offer {
    pub id: String,
    pub listingId: u32,
    pub limit: offer_limit,
    pub price: offer_price,
    pub sellerInformation: offer_seller_information,
    pub restrictions: Vec<String>,
    pub quantities: Vec<u32>,
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct offer_limit {
    min: u32,
    max: u32,
    multiple: u32,
}

#[derive(Debug, Deserialize)]
pub struct offer_price {
    total: u32,
    original: u32,
    commission: u32,
}

#[derive(Debug, Deserialize)]
pub struct offer_seller_information {
    businessType: String,
    affiliationType: String,
}
