use serde_json::Value;

use super::models::FullToriItem;
use crate::error::Error;
use crate::vahti::VahtiItem;

pub fn api_parse_after(json: &str, after: i64) -> Result<Vec<VahtiItem>, Error> {
    let response_json: Value = serde_json::from_str(json)?;
    let mut items = vec![];
    let mut past_weirdness = false;

    if let Some(ads) = response_json["list_ads"].as_array() {
        for ad in ads {
            let ad_object = &ad.as_object().ok_or(Error::Tori)?["ad"];
            let fullitem: FullToriItem = serde_json::from_value(ad_object.to_owned())?;
            let item = VahtiItem::from(fullitem);

            if item.published <= after {
                if past_weirdness {
                    break;
                }
                continue;
            } else {
                past_weirdness = true;
            }

            items.push(item);
        }
    }
    debug!("Parsed {} items", items.len());
    Ok(items)
}
