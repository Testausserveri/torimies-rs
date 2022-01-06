use chrono::{Local, TimeZone};
use serenity::http::Http;
use serenity::model::interactions::message_component::ButtonStyle;
use serenity::utils::Color;

use crate::models::Vahti;
use crate::vahti::VahtiItem;
use crate::Arc;

impl Vahti {
    pub async fn send_updates(
        &self,
        http: Arc<Http>,
        items: Vec<VahtiItem>,
    ) -> Result<(), anyhow::Error> {
        info!("Sending {} items to the user {}", items.len(), self.user_id);
        let user = http.get_user(self.user_id.try_into()?).await?;
        match self.site_id {
            1 => {
                // Tori
                for item in items {
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
                                    b.label("Avaa Hakusivu");
                                    b.style(ButtonStyle::Link);
                                    b.url(&self.url)
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
            }
            2 => {
                // Huutonet
                for item in items {
                    user.dm(&http, |m| {
                        m.embed(|e| {
                            e.color(Color::BLUE);
                            e.description(format!("[{}]({})", item.title, item.url));
                            e.field("Hinta", format!("{} €", item.price), true);
                            e.field(
                                "Myyjä",
                                format!(
                                    "[{}](https://www.huuto.net/kayttaja/{})",
                                    &item.seller_name, item.seller_id
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
                                    b.label("Avaa Hakusivu");
                                    b.style(ButtonStyle::Link);
                                    b.url(&self.url)
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
            }
            _ => bail!("Cannot send updates for unknown site!"),
        }
        Ok(())
    }
}
