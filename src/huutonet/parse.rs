use serde_json::Value;

use super::models::FullHuutonetItem;
use crate::error::Error;
use crate::vahti::VahtiItem;

pub fn api_parse_after(search: &str, after: i64) -> Result<Vec<VahtiItem>, Error> {
    let response_json: Value = serde_json::from_str(search)?;
    let mut items = Vec::new();
    if let Some(ads) = response_json["items"].as_array() {
        for ad in ads {
            let fullitem: FullHuutonetItem = serde_json::from_value(ad.to_owned()).unwrap();
            let item = VahtiItem::from(fullitem);
            if item.published <= after {
                break;
            }
            items.push(item);
        }
    }
    debug!("Parsed {} items", items.len());
    Ok(items)
}
