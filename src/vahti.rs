use std::sync::{Arc, LazyLock, Mutex};

use async_trait::async_trait;
use futures::stream::{self, StreamExt};
use itertools::Itertools;
use regex::Regex;

use crate::database::Database;
use crate::delivery::perform_delivery;
use crate::error::Error;
use crate::huutonet::vahti::HuutonetVahti;
use crate::itemhistory::ItemHistory;
use crate::models::DbVahti;
use crate::tori::vahti::ToriVahti;
use crate::Torimies;

static SITES: LazyLock<Vec<(&LazyLock<Regex>, i32)>> = LazyLock::new(|| {
    vec![
        #[cfg(feature = "tori")]
        (&crate::tori::vahti::TORI_REGEX, crate::tori::ID),
        #[cfg(feature = "huutonet")]
        (&crate::huutonet::vahti::HUUTONET_REGEX, crate::huutonet::ID),
    ]
});

// This is the Vahti trait, implementing it (and a couple of other things)
// provides support for a new site
#[async_trait]
pub trait Vahti
where
    Self: Sized,
{
    async fn update(&mut self, db: &Database) -> Result<Vec<VahtiItem>, Error>;
    async fn validate_url(&self) -> Result<bool, Error>;
    async fn new_db(db: Database, url: &str, userid: u64) -> Result<(), Error>;
    fn is_valid_url(&self, url: &str) -> bool;
    fn from_db(v: DbVahti) -> Result<Self, Error>;
    fn to_db(&self) -> DbVahti;
}

#[derive(Clone, Debug, PartialEq)]
pub struct VahtiItem {
    pub deliver_to: Option<u64>,
    pub delivery_method: Option<i32>,
    pub site_id: i32,
    pub title: String,
    pub vahti_url: Option<String>,
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

pub async fn new_vahti(db: Database, url: &str, userid: u64) -> Result<String, Error> {
    let Some(site_id) = SITES
        .iter()
        .find(|(r, _)| r.is_match(url))
        .map(|(_, sid)| *sid) else {
        return Err(Error::UnknownUrl(url.to_string()));
    };

    if db.fetch_vahti(url, userid as i64).await.is_ok() {
        info!("Not adding a pre-defined Vahti {} for user {}", url, userid);
        return Err(Error::VahtiExists);
    }

    match db.add_vahti_entry(url, userid as i64, site_id).await {
        Ok(_) => Ok(String::from("Vahti added succesfully")),
        Err(e) => Err(e),
    }
}

pub async fn remove_vahti(db: Database, url: &str, userid: u64) -> Result<String, Error> {
    if db.fetch_vahti(url, userid as i64).await.is_err() {
        info!("Not removing a nonexistant vahti!");
        return Ok(
            "Kyseistä vahtia ei ole määritelty, tarkista että kirjoitit linkin oikein".to_string(),
        );
    }
    match db.remove_vahti_entry(url, userid as i64).await {
        Ok(_) => Ok("Vahti poistettu!".to_string()),
        Err(e) => Err(e),
    }
}

impl Torimies {
    pub async fn update_all_vahtis(&mut self) -> Result<(), Error> {
        let vahtis = self.database.fetch_all_vahtis().await?;
        self.update_vahtis(vahtis).await?;
        Ok(())
    }

    pub async fn update_vahtis(&mut self, vahtis: Vec<DbVahti>) -> Result<(), Error> {
        info!("Updating {} vahtis", vahtis.len());
        let start = std::time::Instant::now();

        let ihs = self.itemhistorystorage.clone();

        let db = self.database.clone();
        let dm = self.delivery.clone();

        // FIXME: Not a very good solution
        for vahti in &vahtis {
            if vahti.site_id == crate::tori::ID && ihs.get(&vahti.id).is_none() {
                let ih = Arc::new(Mutex::new(ItemHistory::new()));

                ihs.insert(vahti.id, ih);
            }
        }

        let items = stream::iter(vahtis.iter().cloned())
            .map(|v| (v, ihs.clone(), db.clone()))
            .map(async move |(v, ihs, db)| match v.site_id {
                #[cfg(feature = "tori")]
                crate::tori::ID => {
                    let vid = v.id;
                    let Ok(mut tv) = ToriVahti::from_db(v) else {
                            return vec![];
                        };
                    tv.itemhistory = ihs.get(&vid).map(|ih| ih.clone());
                    if let Ok(is) = tv.update(&db).await {
                        is
                    } else {
                        vec![]
                    }
                }
                #[cfg(feature = "huutonet")]
                crate::huutonet::ID => {
                    if let Ok(mut hv) = HuutonetVahti::from_db(v) {
                        if let Ok(is) = hv.update(&db).await {
                            is
                        } else {
                            vec![]
                        }
                    } else {
                        vec![]
                    }
                }
                i => panic!("Unsupported site_id {}", i),
            })
            .buffer_unordered(50)
            .collect::<Vec<_>>()
            .await;

        let groups: Vec<Vec<VahtiItem>> = items
            .iter()
            .flatten()
            .group_by(|v| {
                (
                    v.deliver_to.expect("bug: impossible"),
                    v.delivery_method.expect("bug: impossible"),
                )
            })
            .into_iter()
            .map(|(_, g)| g.cloned().collect())
            .collect();

        stream::iter(
            groups
                .iter()
                .map(|v| (v, db.clone()))
                .map(async move |(v, db)| {
                    let mut v = v.clone();

                    if let Some(fst) = v.first() {
                        // NOTE: If db fails, blacklisted sellers are not filtered out
                        if let Ok(bl) = db
                            .fetch_user_blacklist(fst.deliver_to.expect("bug: impossible") as i64)
                            .await
                        {
                            v.retain(|i| !bl.contains(&(i.seller_id, i.site_id)));
                        }
                    }
                    v
                })
                .map(|v| (v, dm.clone()))
                .map(async move |(v, dm)| perform_delivery(dm, v.await.clone()).await),
        )
        .buffer_unordered(50)
        .collect::<Vec<_>>()
        .await;

        info!("Update took {}ms", start.elapsed().as_millis());
        Ok(())
    }
}
