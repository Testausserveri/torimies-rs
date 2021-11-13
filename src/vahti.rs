use serenity::{client::Context,http::Http};
use crate::Database;
use crate::tori::parse::*;
use tracing::info;

use std::sync::Arc;

#[derive(Clone)]
pub struct Vahti {
    pub url: String,
    pub user_id: i64,
    pub last_updated: i64,
}

pub async fn new_vahti(ctx: &Context, url: &str, userid: u64) -> String {
    let db = ctx.data.read().await.get::<Database>().unwrap().clone();
    if db.fetch_vahti(url, userid.try_into().unwrap()).await.is_ok() {
        info!("Not adding a pre-defined Vahti {} for user {}", url, userid);
        return "Vahti on jo määritelty!".to_string()
    }
    match db.add_vahti_entry(url, userid.try_into().unwrap()).await {
        Ok(_) => "Vahti lisätty!".to_string(),
        Err(_) => "Virhe tapahtui vahdin lisäyksessä!".to_string(),
    }
}

pub async fn update_all_vahtis(db: Arc<Database>, http: &Http) {
    update_vahtis(db.clone(),http, db.fetch_all_vahtis().await.unwrap()).await;
}

pub async fn update_vahtis(db: Arc<Database>, http: &Http, vahtis: Vec<Vahti>) {
    let mut currenturl = "".to_string();
    let mut currentitems = Vec::new();
    for vahti in vahtis {
        if currenturl != vahti.url {
            currenturl = vahti.url.clone();
            let html = reqwest::get(&currenturl).await.unwrap().text().await.unwrap();
            currentitems = parse_after(html, vahti.last_updated).await
        }
        db.vahti_updated(vahti.clone()).await.unwrap();
        for item in &currentitems {
            let user = http.get_user(vahti.user_id.try_into().unwrap()).await.unwrap();
            user.dm(http, |m| {
                m.embed(|e| {
                    e.color(serenity::utils::Color::DARK_GREEN);
                    e.description(
                        format!("[{}]({})", item.title, item.url)
                    );
                    e.field(
                        "Hinta",
                        format!("{} €",item.price),
                        true,
                    );
                    e.field(
                        "Ilmoitus Jätetty",
                        item.published.to_string(),
                        true,
                    );
                    e.image(item.img_url.clone())
                })
            }).await.unwrap();
        }
    }
}
