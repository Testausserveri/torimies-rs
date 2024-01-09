use encoding::all::ISO_8859_2;
use encoding::{DecoderTrap, Encoding};
use serde_json::Value;
use url::Url;

const TORI_PRICES: [&str; 9] = ["0", "25", "50", "75", "100", "250", "500", "1000", "2000"];

// NOTE: Couldn't find a good crate to do this
fn url_decode(url: &str) -> String {
    let mut result = String::new();
    let mut chars = url.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '%' {
            if chars.peek() == Some(&'%') {
                result.push('%');
                let _ = chars.next();
            } else {
                let hex_str = (&mut chars).take(2).collect::<String>();
                let bytes = hex::decode(&hex_str).unwrap();
                result.push_str(&ISO_8859_2.decode(&bytes, DecoderTrap::Ignore).unwrap())
            }
        } else {
            result.push(c)
        }
    }

    result
}

// TODO: Error handling
pub fn vahti_to_api(vahti: &str) -> String {
    let url = Url::parse(&url_decode(vahti)).unwrap();
    let orig_params = url
        .query_pairs()
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect::<Vec<_>>();

    let mut range_start = None;
    let mut range_end = None;

    let mut params = orig_params
        .clone()
        .into_iter()
        .filter_map(|(k, v)| match k.as_str() {
            "q" => Some((k, v.replace(' ', "+"))),
            "cg" => {
                if orig_params.iter().any(|(k, _)| k == "c") || v == "0" {
                    None
                } else {
                    Some((String::from("category"), v))
                }
            }
            "c" => Some((String::from("category"), v)),
            "ps" => {
                if let Ok(n) = v.parse::<usize>() {
                    range_start = Some(TORI_PRICES[n])
                }
                None
            }
            "pe" => {
                if let Ok(n) = v.parse::<usize>() {
                    range_end = Some(TORI_PRICES[n])
                }
                None
            }
            "ca" => {
                match orig_params
                    .iter()
                    .find(|(k, _)| k == "w")
                    .map(|(_, v)| v.parse::<u64>().ok().map(|n| n > 100))
                {
                    Some(Some(true)) => None,
                    _ => Some((String::from("region"), v)),
                }
            }
            "w" => {
                if let Ok(n) = v.parse::<u64>() {
                    if n > 100 {
                        Some((String::from("region"), (n - 100).to_string()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            "m" => Some((String::from("area"), v)),
            "f" => match v.as_str() {
                "p" => Some((String::from("company_ad"), String::from("0"))),
                "c" => Some((String::from("company_ad"), String::from("1"))),
                _ => None,
            },
            "st" => Some((String::from("ad_type"), v)),
            _ => None,
        })
        .collect::<Vec<_>>();

    if range_start.is_some() || range_end.is_some() {
        params.push((
            String::from("suborder"),
            format!(
                "{}-{}",
                range_start.unwrap_or_default(),
                range_end.unwrap_or_default()
            ),
        ));
    }

    format!(
        "https://api.tori.fi/api/v1.2/public/ads?{}",
        params
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("&")
    )
}

pub async fn is_valid_url(url: &str) -> bool {
    let url = vahti_to_api(url) + "&lim=0";
    let response = reqwest::get(&url)
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();
    if let Some(counter_map) = response["counter_map"].as_object() {
        if let Some(amount) = counter_map["all"].as_i64() {
            amount > 0
        } else {
            false
        }
    } else {
        false
    }
}
