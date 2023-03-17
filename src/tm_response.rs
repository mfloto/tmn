use serde::Deserialize;

///Holds information about a ticketmaster event
#[derive(Debug, Deserialize, Clone)]
pub struct TMRes {
    pub offers: Vec<Offer>,
}

///Holds information about a resale offer
#[derive(Debug, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct Offer {
    pub id: String,
    pub listingId: u32,
    pub limit: OfferLimit,
    pub price: OfferPrice,
    pub sellerInformation: OfferSellerInformation,
    pub restrictions: Vec<String>,
    pub quantities: Vec<u32>,
    pub r#type: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OfferLimit {
    pub min: u32,
    pub max: u32,
    pub multiple: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OfferPrice {
    pub total: u32,
    pub original: u32,
    pub commission: u32,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct OfferSellerInformation {
    pub businessType: String,
    pub affiliationType: String,
}
