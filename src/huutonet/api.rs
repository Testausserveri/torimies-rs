use serde_json::Value;

pub fn vahti_to_api(vahti: &str) -> String {
    let mut url = String::from("https://api.huuto.net/1.1/items?");
    if vahti.contains('&') {
        // Easy parse
        url += &vahti[vahti.find('?').unwrap() + 1..];
    } else {
        // Difficult parse
        let mut args: Vec<&str> = vahti.split('/').collect();
        let args: Vec<&str> = args.drain(4..).collect();
        for arg in args.chunks_exact(2) {
            url += &format!("&{}={}", arg[0], arg[1]);
        }
    }
    url += "&sort=newest"; // You can never be too sure
    url
}

pub async fn is_valid_url(url: &str) -> bool {
    let url = vahti_to_api(url);
    let response = reqwest::get(&url).await.unwrap().json::<Value>().await.unwrap();
    response["totalCount"].as_i64().unwrap() > 0
}
