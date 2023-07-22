#![allow(dead_code)]
use serde::Deserialize;

use crate::vahti::VahtiItem;

#[derive(Deserialize, Debug, Default)]
struct HuutonetLinks {
    #[serde(rename = "self")]
    self_: String,
    category: String,
    alternative: String,
    images: String,
}

#[derive(Deserialize, Debug, Default)]
struct HuutonetImageLinks {
    #[serde(rename = "self")]
    self_: String,
    thumbnail: String,
    medium: String,
    original: Option<String>, // Always `null` ?
}

#[derive(Deserialize, Debug, Default)]
struct HuutonetImage {
    links: HuutonetImageLinks,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct FullHuutonetItem {
    links: HuutonetLinks,
    id: i64,
    title: String,
    category: String,
    seller: String,
    seller_id: i32,
    current_price: f64,
    buy_now_price: Option<f64>,
    sale_method: String,
    list_time: String,
    postal_code: Option<String>,
    location: String,
    closing_time: String,
    bidder_count: i64,
    offer_count: i64,
    has_reserve_price: bool,
    has_reserve_price_exceeded: bool,
    // upgrades: Seems to be an empy vec
    images: Vec<HuutonetImage>,
}

impl From<FullHuutonetItem> for VahtiItem {
    fn from(h: FullHuutonetItem) -> VahtiItem {
        let published = chrono::DateTime::parse_from_str(&h.list_time, "%FT%T%:z")
            .unwrap()
            .timestamp();
        let mut img_url = String::new();
        if !h.images.is_empty() {
            img_url = h.images[0].links.medium.clone();
        }
        VahtiItem {
            delivery_method: None,
            vahti_url: None,
            deliver_to: None,
            site_id: 2,
            title: h.title,
            url: h.links.alternative,
            img_url,
            published,
            price: h.current_price.round() as i64,
            seller_name: h.seller,
            seller_id: h.seller_id,
            location: h.location,
            ad_type: h.sale_method,
            ad_id: h.id,
        }
    }
}
