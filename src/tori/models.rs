#![allow(dead_code)]
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Default)]
struct ToriAccount {
    code: String,
    label: String,
}

#[derive(Deserialize, Debug, Clone, Default)]
struct ToriCategory {
    code: String,
    label: String,
    name: String,
    path_en: String,
    parent: String,
}

#[derive(Deserialize, Debug, Clone, Default)]
struct ToriImage {
    base_url: String,
    media_id: String,
    path: String,
    width: i64,
    height: i64,
}

#[derive(Deserialize, Debug, Clone, Default)]
struct ToriListPrice {
    #[serde(default)]
    currency: String,
    price_value: i64,
    label: String,
}

#[derive(Deserialize, Debug, Clone, Default)]
struct ToriMcSettings {
    use_form: bool,
}

#[derive(Deserialize, Debug, Clone, Default)]
struct ToriListType {
    code: String,
    label: String,
}

#[derive(Deserialize, Debug, Clone, Default)]
struct ToriUserAccount {
    name: String,
    created: String,
}

#[derive(Deserialize, Debug, Clone, Default)]
struct ToriUser {
    account: ToriUserAccount,
    uuid: String,
}

#[derive(Deserialize, Debug, Clone, Default)]
struct ToriPivo {
    enabled: bool,
}

#[derive(Deserialize, Debug, Clone, Default)]
struct ToriListTime {
    label: String,
    value: i64,
}

#[derive(Deserialize, Debug, Clone, Default)]
struct ToriLocation {
    code: String,
    key: String,
    label: String,
    #[serde(default)]
    locations: Vec<ToriLocation>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FullToriItem {
    account: ToriAccount,
    #[serde(default)]
    account_ads: ToriAccount,
    ad_id: String,
    #[serde(default)]
    body: String,
    #[serde(default)]
    category: ToriCategory,
    #[serde(default)]
    company_ad: bool,
    //ad_details: Value, // Complicated and not used anyways
    #[serde(default)]
    full_details: bool,
    #[serde(default)]
    images: Vec<ToriImage>,
    #[serde(default)]
    list_id: String,
    #[serde(default)]
    list_id_code: String,
    list_price: ToriListPrice,
    locations: Vec<ToriLocation>,
    #[serde(default)]
    mc_settings: ToriMcSettings,
    #[serde(default)]
    phone_hidden: bool,
    #[serde(default)]
    prices: Vec<ToriListPrice>,
    #[serde(default)]
    status: String,
    subject: String,
    thumbnail: Option<ToriImage>,
    r#type: ToriListType,
    user: ToriUser,
    share_link: String,
    #[serde(default)]
    pivo: ToriPivo,
    list_time: ToriListTime,
}

#[derive(Clone, Debug)]
pub struct ToriItem {
    pub title: String,
    pub url: String,
    pub img_url: String,
    pub published: i64,
    pub price: i64,
    pub seller_name: String,
    pub seller_id: i32,
    pub location: String,
    pub ad_type: String,
    pub ad_id: i64,
}

impl From<FullToriItem> for ToriItem {
    fn from(t: FullToriItem) -> ToriItem {
        let img_url = match t.thumbnail {
            Some(i) => {
                format!(
                    "https://images.tori.fi/api/v1/imagestori/images{}?rule=medium_660",
                    &i.path[i.path.find('/').unwrap_or(i.path.len())..]
                )
            }
            None => String::new(),
        };
        let mut location_vec: Vec<String> = Vec::new();
        let mut loc = &t.locations[0];
        loop {
            location_vec.push(loc.label.clone());
            if loc.locations.is_empty() {
                break;
            }
            loc = &loc.locations[0];
        }
        let mut prevloc = String::new();
        let mut location = String::new();
        for loc_string in location_vec.iter().rev() {
            if *loc_string == prevloc {
                break;
            }
            prevloc = loc_string.to_string();
            if location.is_empty() {
                location += loc_string;
            } else {
                location += &format!(", {}", loc_string);
            }
        }
        ToriItem {
            title: t.subject,
            url: t.share_link,
            img_url,
            published: t.list_time.value,
            price: t.list_price.price_value,
            seller_name: t.user.account.name,
            seller_id: t.account.code.parse().unwrap(),
            location,
            ad_type: t.r#type.label,
            ad_id: t.ad_id[t.ad_id.rfind('/').unwrap() + 1..].parse().unwrap(),
        }
    }
}
