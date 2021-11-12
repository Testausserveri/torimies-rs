use serenity::client::Context;
use crate::TorimiesData;
use crate::Database;
use crate::tori::parse::*;
use tracing::info;

pub struct Vahti {
    pub url: String,
    pub user_id: i64,
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

pub async fn update_all_vahtis(ctx: &Context) {
    let db = ctx.data.read().await.get::<Database>().unwrap().clone();
    update_vahtis(ctx, db.fetch_all_vahtis().await.unwrap()).await;
}

pub async fn update_vahtis(ctx: &Context, vahtis: Vec<Vahti>) {
    let mut data = ctx.data.write().await;
    let mut torimiesdata = data.get_mut::<TorimiesData>().unwrap();
    let mut currenturl = "".to_string();
    let mut currentitems = Vec::new();
    for vahti in vahtis {
        if currenturl != vahti.url {
            currenturl = vahti.url.clone();
            currentitems = parse_after(&currenturl, Some(torimiesdata.last_updated)).await
        }
        for item in &currentitems {
            let user = ctx.http.get_user(vahti.user_id.try_into().unwrap()).await.unwrap();
            user.dm(&ctx.http, |m| {
                m.embed(|e| {
                    e.color(serenity::utils::Color::DARK_GREEN);
                    e.description(
                        format!("[{}]({})", item.title, item.url)
                    );
                    e.field(
                        "Hinta",
                        item.price.to_string(),
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
    torimiesdata.last_updated = chrono::Local::now().naive_local();
}
