use std::sync::Arc;

use async_trait::async_trait;
use chrono::{Local, TimeZone};
use futures::stream::{self, StreamExt};
use serenity::builder::CreateEmbed;
use serenity::http::Http;
use serenity::model::prelude::component::ButtonStyle;
use serenity::utils::Color;

use crate::delivery::Delivery;
use crate::error::Error;
use crate::vahti::VahtiItem;

pub const ID: i32 = 1;
pub const NAME: &str = "discord";

pub struct Discord {
    pub http: Arc<Http>,
}

/// This is the discord delivery client
/// There will be a separate client for handling commands
impl Discord {
    pub async fn init() -> Result<Self, Error> {
        let token =
            std::env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in the environment");

        // NOTE: We don't need a serenity::Client because we don't have to listen to events.
        let http = Arc::new(Http::new(&token));
        Ok(Self { http })
    }

    pub async fn destroy(self) {}
}

impl VahtiItem {
    fn embed(self, e: &mut CreateEmbed) -> &mut CreateEmbed {
        match self.site_id {
            #[cfg(feature = "tori")]
            crate::tori::ID => {
                let color = match self.ad_type.as_str() {
                    "Myydään" => Color::DARK_GREEN,
                    "Annetaan" => Color::BLITZ_BLUE,
                    _ => Color::FADED_PURPLE,
                };

                e.color(color);
                e.description(format!("[{}]({})", self.title, self.url));
                e.field("Hinta", format!("{} €", self.price), true);
                e.field(
                    "Myyjä",
                    format!(
                        "[{}](https://www.tori.fi/li?&aid={})",
                        self.seller_name, self.seller_id
                    ),
                    true,
                );
                e.field("Sijainti", &self.location, true);
                e.field(
                    "Ilmoitus Jätetty",
                    Local
                        .timestamp_opt(self.published, 0)
                        .unwrap()
                        .format("%d/%m/%Y %R"),
                    true,
                );
                e.field("Ilmoitustyyppi", self.ad_type.to_string(), true);
                e.footer(|f| f.text(self.vahti_url.expect("bug: impossible")));
                if !self.img_url.is_empty() {
                    e.image(&self.img_url);
                }
            }
            #[cfg(feature = "huutonet")]
            crate::huutonet::ID => {
                e.color(Color::BLUE);
                e.description(format!("[{}]({})", self.title, self.url));
                e.field("Hinta", format!("{} €", self.price), true);
                e.field(
                    "Myyjä",
                    format!(
                        "[{}](https://www.huuto.net/kayttaja/{})",
                        &self.seller_name, self.seller_id
                    ),
                    true,
                );
                e.field("Sijainti", &self.location, true);
                e.field(
                    "Ilmoitus Jätetty",
                    Local
                        .timestamp_opt(self.published, 0)
                        .unwrap()
                        .format("%d/%m/%Y %R"),
                    true,
                );
                e.field("Ilmoitustyyppi", self.ad_type.to_string(), true);
                e.footer(|f| f.text(self.vahti_url.expect("bug: impossible")));
                if !self.img_url.is_empty() {
                    e.image(&self.img_url);
                }
            }
            i => panic!("Unsupported site_id {}", i),
        }
        e
    }
}

#[async_trait]
impl Delivery for Discord {
    async fn deliver(&self, items: Vec<VahtiItem>) -> Result<(), Error> {
        let Some(fst) = items.first() else {
            return Ok(());
        };

        assert!(items.iter().all(|i| i.deliver_to == fst.deliver_to));

        info!(
            "Delivering {} items to {}",
            items.len(),
            fst.deliver_to.unwrap()
        );

        // NOTE: Let's try 5 embeds per message, discord has a limit afaik, but idk
        // if the text/character limit will become an issue before the embed limit does
        let chunks: Vec<Vec<VahtiItem>> = items.chunks(5).map(|c| c.to_vec()).collect();

        let http = self.http.clone();
        let recipient = http
            .get_user(fst.deliver_to.expect("bug: impossible"))
            .await?;

        stream::iter(chunks.iter().cloned())
            .map(|is| (is, http.clone(), recipient.clone()))
            .map(async move |(items, http, rec)| {
                rec.dm(&http, |m| {
                    for item in items {
                        m.add_embed(|e| item.clone().embed(e));
                    }
                    #[cfg(feature = "discord-command")]
                    m.components(|c| {
                        c.create_action_row(|r| {
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
                    });
                    m
                })
                .await
                // FIXME: Perhaps don't ignore an error here
                .ok()
            })
            .buffer_unordered(*crate::FUTURES_MAX_BUFFER_SIZE)
            .collect::<Vec<_>>()
            .await;

        Ok(())
    }
}
