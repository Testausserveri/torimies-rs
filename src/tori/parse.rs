use serde_json::Value;

use super::models::FullToriItem;
use crate::vahti::VahtiItem;

pub async fn api_parse_after(search: &str, after: i64) -> Result<Vec<VahtiItem>, anyhow::Error> {
    let response_json: Value = serde_json::from_str(search)?;
    let mut items = Vec::new();
    if let Some(ads) = response_json["list_ads"].as_array() {
        for ad in ads {
            let ad_object = &ad.as_object().ok_or(anyhow!("Tori has done stupiding"))?["ad"];
            let fullitem: FullToriItem = serde_json::from_value(ad_object.to_owned())?;
            let item = VahtiItem::from(fullitem);
            if item.published <= after {
                break;
            }
            items.push(item);
        }
    }
    Ok(items)
}
