use chrono::prelude::*;
use chrono::Duration;
use chrono::NaiveDateTime;
use html_parser::{Dom, Node};

#[derive(Clone)]
pub struct ToriItem {
    pub title: String,
    pub url: String,
    pub img_url: String,
    pub published: NaiveDateTime,
    pub price: i64,
}

pub async fn parse_after(html: String, after: i64) -> Vec<ToriItem> {
    let html = html[html.find(r"<body").unwrap()..html.rfind(r"</body>").unwrap()+7].to_string();
    let dom = Dom::parse(&html).unwrap();
    let iter = dom.children.get(0).unwrap().into_iter();

    let mut items: Vec<ToriItem> = iter.filter_map(|item| match item {
        Node::Element(ref element) => {
            let mut r = None;
            if let Some(k) = &element.id {
                if k.starts_with("item_") {
                    let url = element.attributes["href"].clone().unwrap();
                    let desc_node = element.clone().children.into_iter().find(|e| {
                        if let Node::Element(tt) = e {
                            return tt.classes.contains(&"desc_flex".to_string());
                        };
                        false
                    }).unwrap();
                    let desc = match desc_node {
                        Node::Element(ref e) => e,
                        _ => panic!("No description element!"),
                    };
                    let img_container = match element.clone().children.into_iter().find(|e| {
                        if let Node::Element(ref tt) = e {
                            return tt.classes.contains(&"image_container".to_string());
                        };
                        false
                    }).unwrap() {
                        Node::Element(ref e) => e.clone(),
                        _ => panic!("No image_container element!")
                    };
                    let img_url = match img_container.children.get(1).unwrap() {
                        Node::Element(ref e) => match e.children.get(0) {
                            Some(Node::Element(ref ee)) => {
                                match ee.attributes["src"].as_ref() {
                                    Some(u) => u.clone(),
                                    None => "NoImage".to_string(),
                                }
                            },
                            _ => "".to_string()
                        },
                        _ => "".to_string()
                    };
                    let left = match desc.children.get(0).unwrap() {
                        Node::Element(ref e) => e,
                        _ => panic!("No left-details")
                    };
                    let title = match left.clone().children.get(0).unwrap() {
                        Node::Element(ref e) => {
                            match e.children.get(0).unwrap() {
                                Node::Text(ref s) => s.to_owned(),
                                _ => "".to_string(),
                            }
                        },
                        _ => "".to_string(),
                    };
                    let price = match left.children.get(1).unwrap() {
                        Node::Element(ref e) => {
                            match e.children.get(0).unwrap() {
                                Node::Element(ref ee) => {
                                    match ee.children.get(0) {
                                        Some(Node::Text(ref s)) => s[..s.find(' ').unwrap_or(s.len())].parse::<i64>().unwrap_or(0),
                                        _ => 0
                                    }
                                },
                                _ => 0,
                            }
                        },
                        _ => 0,
                    };
                    let right = match desc.children.get(1).unwrap() {
                        Node::Element(ref e) => e,
                        _ => panic!("No right-details")
                    };
                    let datestring = match right.children.get(0).unwrap() {
                        Node::Element(ref e) => {
                            match e.children.get(0).unwrap() {
                                Node::Element(ref ee) => {
                                    match ee.children.get(0) {
                                        Some(Node::Text(ref s)) => s,
                                        _ => ""
                                    }
                                },
                                _ => "",
                            }
                        },
                        _ => "",
                    };
                    let date = parse_toridate(datestring);
                    r = Some(ToriItem { title, url, img_url, published: date, price });

                }
                else {
                    r = None;
                }
            }
            r
        }
        _ => None,
    }).collect();
    // FIXME: Parsing times between 24 and 02 yelds a -22 hour offset
    items.retain(|item| item.published.timestamp() > after);
    items
}

fn parse_toridate(ds: &str) -> NaiveDateTime {
    let dsplit: Vec<&str> = ds.split_whitespace().collect();
    let m = dsplit[dsplit.len()-2];
    let jotain;
    let ds = ds.replace(m,
        match m {
        "tänään" => {
            jotain = Utc::now().naive_local().format("%d %b").to_string();
            &jotain
        },
        "eilen" => {
            jotain = (Utc::now().naive_local()-Duration::days(1)).format("%d %b").to_string();
            &jotain
        }
        "tam" => "Jan",
        "hel" => "Feb",
        "maa" => "Mar",
        "huh" => "Apr",
        "tou" => "May",
        "kes" => "Jun",
        "hei" => "Jul",
        "elo" => "Aug",
        "syy" => "Sep",
        "lok" => "Oct",
        "mar" => "Nov",
        "jou" => "Dec",
        _ => unreachable!("THIS SHOULD NOT BE A THING: {}", m),
    }).replace('\n', "").replace('\t', "");
    // FIXME: Don't hardcode year here
    let ds = format!("{} 2021", ds);
    match NaiveDateTime::parse_from_str(&ds, "%d %b %H:%M %Y") {
        Ok(a) => a,
        Err(e) => panic!("{}", e),
    }
}
