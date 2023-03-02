use serde_json::Value;

use crate::error::Error;

pub async fn get_seller_name_from_id(sellerid: i32) -> Result<String, Error> {
    let url = format!("https://api.huuto.net/1.1/users/{}", sellerid);
    let response = reqwest::get(&url).await?.text().await?;
    let response_json: Value = serde_json::from_str(&response)?;
    Ok(response_json["username"].to_string())
}
