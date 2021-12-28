use serde_json::Value;

pub fn vahti_to_api(vahti: &str) -> String {
    let mut url = "https://api.tori.fi/api/v1.2/public/ads?".to_owned();
    let args = &vahti[vahti.find('?').unwrap() + 1..];
    let mut price_set = false;
    let mut region_defined = false;
    let mut startprice = "";
    let mut endprice = "";
    let mut api_args = Vec::<(String, String)>::new();
    for arg in args.split('&') {
        let mut parts: Vec<&str> = arg.split('=').collect();
        if parts.len() == 1 {
            parts.push("");
        }
        match parts[0] {
            "ps" => {
                startprice = parts[1];
                price_set = true;
            }
            "pe" => {
                endprice = parts[1];
                price_set = true;
            }
            "cg" => {
                if parts[1] != "0" {
                    api_args.push(("category".to_string(), parts[1].to_string()));
                }
            }
            "st" => api_args.push(("ad_type".to_string(), parts[1].to_string())),
            "m" => api_args.push(("area".to_string(), parts[1].to_string())),
            "w" => {
                let reg: i32 = parts[1].parse().unwrap();
                if reg >= 100 {
                    region_defined = true;
                    api_args.push(("region".to_string(), (reg - 100).to_string()));
                }
            }
            "ca" => api_args.push(("caregion".to_string(), parts[1].to_string())),
            _ => api_args.push((parts[0].to_string(), parts[1].to_string())),
        }
    }
    for arg in api_args {
        if arg.0.is_empty() {
            continue;
        }
        if arg.0 == "caregion" {
            if !region_defined {
                url += &format!("&{}={}", arg.0, arg.1);
            }
        } else {
            url += &format!("&{}={}", arg.0, arg.1);
        }
    }
    url = url.replace("%E4", "ä");
    url = url.replace("%C4", "Ä");
    url = url.replace("%F6", "ö");
    url = url.replace("%D6", "Ö");
    if price_set && !startprice.is_empty() && !endprice.is_empty() {
        url += &format!("&suborder={}-{}", &startprice, &endprice);
    }
    url
}

pub async fn is_valid_url(url: &str) -> bool {
    let url = vahti_to_api(url) + "&lim=0";
    let response = reqwest::get(&url).await.unwrap().text().await.unwrap();
    let response_json: Value = serde_json::from_str(&response).unwrap();
    if let Some(counter_map) = response_json["counter_map"].as_object() {
        if let Some(amount) = counter_map["all"].as_i64() {
            amount > 0
        } else {
            false
        }
    } else {
        false
    }
}
