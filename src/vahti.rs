use std::collections::BTreeMap;
use std::sync::Arc;

use chrono::{Local, TimeZone};
use regex::Regex;
use serenity::client::Context;
use serenity::http::Http;
use serenity::model::interactions::message_component::ButtonStyle;
use serenity::utils::Color;

use crate::extensions::ClientContextExt;
use crate::models::Vahti;
use crate::{Database, ItemHistory, Mutex};

#[derive(Clone, Debug)]
pub struct VahtiItem {
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
            "Kyseistä vahtia ei ole määritelty, tarkista että kirjoitit linkin oikein".to_string(),
        );
    }
    match db.remove_vahti_entry(url, userid.try_into()?).await {
        Ok(_) => Ok("Vahti poistettu!".to_string()),
        Err(_) => bail!("Virhe tapahtui vahdin poistamisessa!".to_string()),
    }
}

fn vahti_to_api(vahti: &str) -> String {
    let tori_regex = Regex::new(r"^https://(m\.|www\.)?tori\.fi/.*\?.*$").unwrap();
    let huuto_regex = Regex::new(r"^https://(www\.)?huuto\.net/haku?.*$").unwrap();
    let url;
    if tori_regex.is_match(vahti) {
        url = crate::tori::api::vahti_to_api(vahti);
    } else if huuto_regex.is_match(vahti) {
        url = crate::huutonet::api::vahti_to_api(vahti);
    } else {
        panic!("Unidentified url in a Vahti: {}", vahti);
    }
    url
}

pub async fn is_valid_url(url: &str) -> bool {
    let tori_regex = Regex::new(r"^https://(m\.|www\.)?tori\.fi/.*\?.*$").unwrap();
    let huuto_regex = Regex::new(r"^https://(www\.)?huuto\.net/haku?.*$").unwrap();
    if tori_regex.is_match(url) {
        return crate::tori::api::is_valid_url(url).await;
    } else if huuto_regex.is_match(url) {
        return crate::huutonet::api::is_valid_url(url).await;
    }
    false
}

pub async fn api_parse_after(text: &str, after: i64) -> Result<Vec<VahtiItem>, anyhow::Error> {
    // BIG FIXME: This is definitely not the best solution
    // Optimally Vahti would be an enum of some kind, and
    // all the stuff would be executed to that enum automatically
    // ex. g. Vahti::parse_after(i64) would execute stuff depending on
    // the vahti type aka for which site the vahti is setup for
    if text.contains("tori.fi") {
        return crate::tori::parse::api_parse_after(text, after).await;
    } else if text.contains("huuto.net") {
        return crate::huutonet::parse::api_parse_after(text, after).await;
    } else {
        error!("Unrecognized vathi result!");
        bail!("Unrecognized vahti result!");
    }
}

pub async fn update_all_vahtis(
    db: Database,
    itemhistory: Arc<Mutex<ItemHistory>>,
    http: Arc<Http>,
) -> Result<(), anyhow::Error> {
    itemhistory.lock().await.purge_old();
    let vahtis = db.fetch_all_vahtis_group().await?;
    update_vahtis(db, itemhistory, http, vahtis).await?;
    Ok(())
}

pub async fn update_vahtis(
    db: Database,
    itemhistory: Arc<Mutex<ItemHistory>>,
    httpt: Arc<Http>,
    vahtis: BTreeMap<String, Vec<(i64, i64)>>,
) -> Result<(), anyhow::Error> {
    for (url, ids) in vahtis {
        let http = httpt.clone();
        let db = db.clone();
        let itemhistory = itemhistory.clone();
        tokio::spawn(async move {
            let res = reqwest::get(vahti_to_api(&url))
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            for (id, last_updated) in ids {
                if let Ok(mut currentitems) = api_parse_after(&res, last_updated).await {
                    if currentitems.len() == 10 {
                        debug!("Unsure on whether we got all the items... Querying for all of them now");
                        currentitems = api_parse_after(
                            &reqwest::get(vahti_to_api(&url))
                                .await
                                .unwrap()
                                .text()
                                .await
                                .unwrap(),
                            last_updated,
                        )
                        .await
                        .unwrap();
                    }
                    if currentitems.is_empty() {
                        continue;
                    }
                    for item in currentitems.iter().rev() {
                        if itemhistory.lock().await.contains(item.ad_id, id) {
                            debug!("Item {} in itemhistory! Skipping!", item.ad_id);
                            continue;
                        }
                        itemhistory.lock().await.add_item(
                            item.ad_id,
                            id,
                            chrono::Local::now().timestamp(),
                        );
                        let blacklist = db.fetch_user_blacklist(id).await.unwrap();
                        if blacklist.contains(&item.seller_id) {
                            info!(
                                "Seller {} blacklisted by user {}! Skipping!",
                                &item.seller_id, id
                            );
                            continue;
                        }
                        let user = http.get_user(id.try_into().unwrap()).await.unwrap();
                        let c = match item.ad_type.as_str() {
                            "Myydään" => Color::DARK_GREEN,
                            "Annetaan" => Color::BLITZ_BLUE,
                            _ => Color::FADED_PURPLE,
                        };
                        user.dm(&http, |m| {
                            m.embed(|e| {
                                e.color(c);
                                e.description(format!("[{}]({})", item.title, item.url));
                                e.field("Hinta", format!("{} €", item.price), true);
                                e.field(
                                    "Myyjä",
                                    format!(
                                        "[{}](https://www.tori.fi/li?&aid={})",
                                        item.seller_name, item.seller_id
                                    ),
                                    true,
                                );
                                e.field("Sijainti", &item.location, true);
                                e.field(
                                    "Ilmoitus Jätetty",
                                    Local.timestamp(item.published, 0).format("%d/%m/%Y %R"),
                                    true,
                                );
                                e.field("Ilmoitustyyppi", item.ad_type.to_string(), true);
                                if !item.img_url.is_empty() {
                                    e.image(&item.img_url);
                                }
                                e
                            });
                            m.components(|c| {
                                c.create_action_row(|r| {
                                    r.create_button(|b| {
                                        b.label("Avaa ilmoitus");
                                        b.style(ButtonStyle::Link);
                                        b.url(&item.url)
                                    });
                                    r.create_button(|b| {
                                        b.label("Hakulinkki");
                                        b.style(ButtonStyle::Link);
                                        b.url(&url)
                                    });
                                    r.create_button(|b| {
                                        b.label("Estä myyjä");
                                        b.style(ButtonStyle::Danger);
                                        b.custom_id("block_seller")
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
                    db.vahti_updated(
                        Vahti {
                            user_id: id,
                            url: url.clone(),
                            id: 0,
                            last_updated,
                        },
                        Some(currentitems[0].published),
                    )
                    .await
                    .unwrap();
                }
            }
        });
    }
    Ok(())
}
