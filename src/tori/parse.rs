use serde_json::Value;

#[derive(Clone)]
pub struct ToriItem {
    pub title: String,
    pub url: String,
    pub img_url: String,
    pub published: i64,
    pub price: i64,
    pub seller_name: String,
    pub location: String,
    pub ad_type: String,
    pub ad_id: i64,
}

pub async fn api_parse_after(search: &str, after: i64) -> Vec<ToriItem> {
    // TODO: This currently only supports the search term (and maybe something else I'm not sure)
    let response_json: Value = serde_json::from_str(search).unwrap();
    let mut items = Vec::new();
    if let Some(ads) = response_json["list_ads"].as_array() {
        for ad in ads.to_owned() {
            let ad_object = ad.as_object().unwrap()["ad"].clone();
            let title = ad_object.clone()["subject"]
                .as_str()
                .unwrap_or("")
                .to_string();
            let price = ad_object.clone()["list_price"]["price_value"]
                .as_i64()
                .unwrap_or(0);
            let published = ad_object.clone()["list_time"]["value"].as_i64().unwrap();
            let url = ad_object.clone()["share_link"]
                .as_str()
                .unwrap()
                .to_string();
            let img_path = ad_object.clone()["thumbnail"]["path"]
                .as_str()
                .unwrap_or("")
                .to_string();
            let img_url = format!(
                "https://images.tori.fi/api/v1/imagestori/images{}?rule=medium_660",
                &img_path[img_path.find('/').unwrap_or_else(|| img_path.len())..]
            );
            let seller_name = ad_object.clone()["user"]["account"]["name"]
                .as_str()
                .unwrap_or("Unknown Seller")
                .to_string();
            let region = ad_object.clone()["locations"][0]["label"]
                .as_str()
                .unwrap_or("Unknown region")
                .to_string();
            let locality = ad_object.clone()["locations"][0]["locations"][0]["label"]
                .as_str()
                .unwrap_or("Unknown city")
                .to_string();
            let location = format!("{}, {}", locality, region);
            let ad_type = ad_object.clone()["type"]["label"]
                .as_str()
                .unwrap_or("")
                .to_string();
            let ad_id = ad_object.clone()["list_id_code"]
                .as_str()
                .unwrap()
                .parse::<i64>()
                .unwrap();
            items.push(ToriItem {
                title,
                url,
                img_url,
                published,
                price,
                seller_name,
                location,
                ad_type,
                ad_id,
            });
        }
    }
    items.retain(|item| item.published > after);
    items
}
