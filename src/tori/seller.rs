use serde_json::Value;

pub async fn get_seller_name_from_id(sellerid: i64) -> Result<String, anyhow::Error> {
    let url = format!("https://api.tori.fi/api/v1.2/public/ads?account={}&lim=1", sellerid);
    let response = reqwest::get(&url).await?.text().await?;
    let response_json: Value = serde_json::from_str(&response)?;
    if let Some(ads) = response_json["list_ads"].as_array() {
        if ads.is_empty() {
            return Ok(String::from("Unknown Seller"))
        }
        return Ok(ads[0].as_object().unwrap()["ad"]["user"]["account"]["name"].as_str().unwrap_or("Unknown Seller").to_string())
    }
    Ok(String::from("Unknown Seller"))
}
