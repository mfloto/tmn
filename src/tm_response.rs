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
    pub min: u32,
    pub max: u32,
    pub multiple: u32,
}

#[derive(Debug, Deserialize)]
pub struct offer_price {
    pub total: u32,
    pub original: u32,
    pub commission: u32,
}

#[derive(Debug, Deserialize)]
pub struct offer_seller_information {
    pub businessType: String,
    pub affiliationType: String,
}
