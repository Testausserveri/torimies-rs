use std::sync::Arc;

use chrono::{Local, TimeZone};
use serde_json::Value;
use serenity::client::Context;
use serenity::http::Http;
use serenity::model::interactions::message_component::ButtonStyle;
use serenity::utils::Color;

use crate::extensions::ClientContextExt;
use crate::tori::parse::*;
use crate::{Database, ItemHistory, Mutex};

#[derive(Clone)]
pub struct Vahti {
    pub url: String,
    pub user_id: i64,
    pub last_updated: i64,
}

pub async fn new_vahti(ctx: &Context, url: &str, userid: u64) -> Result<String, anyhow::Error> {
    let db = ctx.get_db().await?;
    if db.fetch_vahti(url, userid.try_into()?).await.is_ok() {
        info!("Not adding a pre-defined Vahti {} for user {}", url, userid);
        return Ok("Vahti on jo määritelty!".to_string());
    }
    match db.add_vahti_entry(url, userid.try_into()?).await {
        Ok(_) => Ok("Vahti lisätty!".to_string()),
        Err(_) => bail!("Virhe tapahtui vahdin lisäyksessä!"),
    }
}

pub async fn remove_vahti(ctx: &Context, url: &str, userid: u64) -> Result<String, anyhow::Error> {
    let db = ctx.get_db().await?;
    if db.fetch_vahti(url, userid.try_into()?).await.is_err() {
        info!("Not removing a nonexistant vahti!");
        return Ok(
            "Kyseistä vahtia ei ole määritelyt, tarkista että kirjoiti linkin oikein".to_string(),
        );
    }
    match db.remove_vahti_entry(url, userid.try_into()?).await {
        Ok(_) => Ok("Vahti poistettu!".to_string()),
        Err(_) => bail!("Virhe tapahtui vahdin poistamisessa!".to_string()),
    }
}

fn vahti_to_api(vahti: &str) -> String {
    let mut url = "https://api.tori.fi/api/v1.2/public/ads".to_owned()
        + &vahti.to_owned()[vahti.find('?').unwrap()..];
    let mut startprice: String = "".to_string();
    let mut endprice: String = "".to_string();
    let mut price_set = false;
    if let Some(index) = url.find("ps=") {
        let endindex = url[index..].find('&').unwrap_or(url.len() - index);
        startprice = url[index + 3..endindex + index].to_string();
        price_set = true;
    }
    if let Some(index) = url.find("pe=") {
        let endindex = url[index..].find('&').unwrap_or(url.len() - index);
        endprice = url[index + 3..endindex + index].to_string();
        price_set = true;
    }
    url = url.replace("cg=", "category=");
    // because in the API category=0 yealds no results and in the search it just means
    // no category was specified
    url = url.replace("&category=0", "");
    if let Some(index) = url.find("w=") {
        let region;
        let endindex = url[index..].find('&').unwrap_or(url.len() - index);
        let num = url[index + 2..endindex + index].parse::<i32>().unwrap();
        if num >= 100 {
            region = num - 100;
            url = url.replace(&url[index..endindex + index], &format!("region={}", region));
        } else if url.contains("ca=") {
            let nindex = url.find("ca=").unwrap();
            let nendindex = url[nindex..].find('&').unwrap_or(url.len() - nindex);
            let num = &url[nindex + 3..nendindex + nindex];
            url = url.replace(&url[index..endindex + index], &format!("region={}", num));
        }
    } else {
        url = url.replace("ca=", "region=");
    }
    url = url.replace("st=", "ad_type=");
    url = url.replace("m=", "area=");
    url = url.replace("_s", ""); // FIXME: not a good solution
    url = url.replace(" ", "+");
    url = url.replace("%E4", "ä");
    url = url.replace("%C4", "Ä");
    url = url.replace("%F6", "ö");
    url = url.replace("%D6", "Ö");
    if price_set {
        url = url + &format!("&suborder={}-{}", &startprice, &endprice);
    }
    url
}

pub async fn is_valid_url(url: &str) -> bool {
    if !url.starts_with("https://www.tori.fi/") {
        return false;
    }
    if !url.contains('?') {
        return false;
    }
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

pub async fn update_all_vahtis(
    db: Arc<Database>,
    itemhistory: &mut Arc<Mutex<ItemHistory>>,
    http: &Http,
) -> Result<(), anyhow::Error> {
    itemhistory.lock().await.purge_old();
    let vahtis = db.fetch_all_vahtis().await?;
    update_vahtis(db, itemhistory, http, vahtis).await?;
    Ok(())
}

pub async fn update_vahtis(
    db: Arc<Database>,
    itemhistory: &mut Arc<Mutex<ItemHistory>>,
    http: &Http,
    vahtis: Vec<Vahti>,
) -> Result<(), anyhow::Error> {
    let mut currenturl = String::new();
    let mut currentitems = Vec::new();
    let test = std::time::Instant::now();
    for vahtichunks in vahtis.chunks(5) {
        //TODO: Make networking better
        let vahtichunks = vahtichunks.iter().zip(vahtichunks.iter().map(|vahti| {
            if currenturl != vahti.url {
                currenturl = vahti.url.clone();
                let url = vahti_to_api(&currenturl);
                info!("Sending query: {}&lim=10", url);
                let response = reqwest::get(url + "&lim=10");
                Some(response)
            } else {
                None
            }
        }));
        for (vahti, request) in vahtichunks {
            if let Some(req) = request {
                currentitems =
                    api_parse_after(&req.await?.text().await?.clone(), vahti.last_updated).await?;
                if currentitems.len() == 10 {
                    info!("Unsure on whether we got all the items... Querying for all of them now");
                    currentitems = api_parse_after(
                        &reqwest::get(vahti_to_api(&vahti.url))
                            .await?
                            .text()
                            .await?
                            .clone(),
                        vahti.last_updated,
                    )
                    .await?
                }
            }
            if !currentitems.is_empty() {
                db.vahti_updated(vahti.clone(), Some(currentitems[0].published))
                    .await
                    .unwrap();
            }
            info!("Got {} items", currentitems.len());
            for item in currentitems.iter().rev() {
                if itemhistory.lock().await.contains(item.ad_id, vahti.user_id) {
                    info!("Item {} in itemhistory! Skipping!", item.ad_id);
                    continue;
                }
                itemhistory.lock().await.add_item(
                    item.ad_id,
                    vahti.user_id,
                    chrono::Local::now().timestamp(),
                );
                let user = http
                    .get_user(vahti.user_id.try_into().unwrap())
                    .await
                    .unwrap();
                let c = match item.ad_type.as_str() {
                    "Myydään" => Color::DARK_GREEN,
                    "Annetaan" => Color::BLITZ_BLUE,
                    _ => Color::FADED_PURPLE,
                };
                user.dm(http, |m| {
                    m.embed(|e| {
                        e.color(c);
                        e.description(format!(
                            "[{}]({})\n[Hakulinkki]({})",
                            item.title, item.url, vahti.url
                        ));
                        e.field("Hinta", format!("{} €", item.price), true);
                        e.field("Myyjä", item.seller_name.clone(), true);
                        e.field("Sijainti", item.location.clone(), true);
                        e.field(
                            "Ilmoitus Jätetty",
                            Local.timestamp(item.published, 0).format("%d/%m/%Y %R"),
                            true,
                        );
                        e.field("Ilmoitustyyppi", item.ad_type.to_string(), true);
                        e.image(item.img_url.clone())
                    });
                    m.components(|c| {
                        c.create_action_row(|r| {
                            r.create_button(|b| {
                                b.label("Avaa ilmoitus");
                                b.style(ButtonStyle::Link);
                                b.url(item.url.clone())
                            });
                            r.create_button(|b| {
                                b.label("Poista Vahti");
                                b.style(ButtonStyle::Danger);
                                b.custom_id("remove_vahti")
                            })
                        })
                    })
                })
                .await
                .unwrap();
            }
        }
    }
    info!("Finished requests in {} ms", test.elapsed().as_millis());
    Ok(())
}
