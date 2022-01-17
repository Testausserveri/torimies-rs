use std::collections::BTreeMap;
use std::sync::Arc;

use regex::Regex;
use serenity::client::Context;
use serenity::http::Http;

use crate::extensions::ClientContextExt;
use crate::models::Vahti;
use crate::{Database, ItemHistory, Mutex};

lazy_static::lazy_static! {
    static ref TORI_REGEX: Regex = Regex::new(r"^https://(m\.|www\.)?tori\.fi/.*\?.*$").unwrap();
    static ref HUUTONET_REGEX: Regex = Regex::new(r"^https://(www\.)?huuto\.net/haku?.*$").unwrap();
}

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

#[derive(Clone, Copy)]
pub enum SiteId {
    Unknown = 0,
    Tori = 1,
    Huutonet = 2,
}

impl From<&str> for SiteId {
    fn from(url: &str) -> SiteId {
        match url {
            _ if TORI_REGEX.is_match(url) => SiteId::Tori,
            _ if HUUTONET_REGEX.is_match(url) => SiteId::Huutonet,
            _ => SiteId::Unknown,
        }
    }
}

impl Vahti {
    fn to_api(&self) -> Result<String, anyhow::Error> {
        // FIXME: I have no idea how to do this properly using enums
        match self.site_id {
            1 => Ok(crate::tori::api::vahti_to_api(&self.url)),
            2 => Ok(crate::huutonet::api::vahti_to_api(&self.url)),
            _ => bail!("Can't interpret a Vahti to an unknown site"),
        }
    }
    async fn parse_after_from_text(&self, text: &str) -> Result<Vec<VahtiItem>, anyhow::Error> {
        // FIXME: I have no idea how to do this properly using enums pt 2
        match self.site_id {
            1 => crate::tori::parse::api_parse_after(text, self.last_updated),
            2 => crate::huutonet::parse::api_parse_after(text, self.last_updated),
            _ => bail!("Can't interpret a Vahti to an unknown site"),
        }
    }
    async fn parse_after(&self) -> Result<Vec<VahtiItem>, anyhow::Error> {
        let res = reqwest::get(self.to_api().unwrap())
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        self.parse_after_from_text(&res).await
    }
    async fn update(self, db: &Database, timestamp: i64) -> Result<usize, anyhow::Error> {
        db.vahti_updated(self, Some(timestamp)).await
    }
}

pub async fn new_vahti(ctx: &Context, url: &str, userid: u64) -> Result<String, anyhow::Error> {
    let db = ctx.get_db().await?;
    if db.fetch_vahti(url, userid.try_into()?).await.is_ok() {
        info!("Not adding a pre-defined Vahti {} for user {}", url, userid);
        return Ok("Vahti on jo määritelty!".to_string());
    }
    match db.add_vahti_entry(url, userid.try_into()?).await {
        Ok(_) => Ok("Vahti lisätty!".to_string()),
        Err(e) => bail!("Virhe tapahtui vahdin lisäyksessä!: {}", e),
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
        Err(e) => bail!("Virhe tapahtui vahdin poistamisessa!: {}", e),
    }
}

pub async fn is_valid_url(url: &str) -> bool {
    if TORI_REGEX.is_match(url) {
        return crate::tori::api::is_valid_url(url).await;
    } else if HUUTONET_REGEX.is_match(url) {
        return crate::huutonet::api::is_valid_url(url).await;
    }
    false
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
    grouped_vahtis: BTreeMap<String, Vec<Vahti>>,
) -> Result<(), anyhow::Error> {
    for (_, vahtis) in grouped_vahtis {
        let http = httpt.clone();
        let db = db.clone();
        let itemhistory = itemhistory.clone();
        tokio::spawn(async move {
            let res = reqwest::get(vahtis[0].to_api().unwrap() + "&lim=10")
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            let site_id = SiteId::from(vahtis[0].url.as_str());
            for vahti in vahtis {
                if let Ok(mut currentitems) = vahti.parse_after_from_text(&res).await {
                    if currentitems.len() == 10 {
                        debug!("Unsure on whether we got all the items... Querying for all of them now");
                        currentitems = vahti.parse_after().await.unwrap();
                    }
                    if currentitems.is_empty() {
                        continue;
                    }
                    let mut items = Vec::new();
                    for item in currentitems.iter().rev() {
                        if itemhistory.lock().await.contains(item.ad_id, vahti.user_id, site_id as i32) {
                            debug!("Item {},{} in itemhistory! Skipping!", item.ad_id, site_id as i32);
                            continue;
                        }
                        itemhistory.lock().await.add_item(
                            item.ad_id,
                            vahti.user_id,
                            site_id as i32,
                            chrono::Local::now().timestamp(),
                            );
                        let blacklist = db.fetch_user_blacklist(vahti.user_id).await.unwrap();
                        if blacklist
                            .contains(&(item.seller_id, site_id as i32))
                        {
                            info!(
                                "Seller {} blacklisted by user {}! Skipping!",
                                &item.seller_id, vahti.user_id
                            );
                            continue;
                        }
                        items.push(item.to_owned());
                    }
                    vahti
                        .send_updates(http.clone(), items.clone())
                        .await
                        .unwrap();
                    vahti.update(&db, currentitems[0].published).await.unwrap();
                }
            }
        });
    }
    Ok(())
}
