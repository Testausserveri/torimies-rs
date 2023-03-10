use itertools::Itertools;
use serenity::builder::CreateComponents;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::prelude::*;

use super::extensions::ClientContextExt;

pub fn menu_from_options<'a>(
    components: &'a mut CreateComponents,
    custom_id: &'a str,
    options: Vec<(impl ToString, impl ToString)>,
) -> &'a mut CreateComponents {
    components.create_action_row(|r| {
        r.create_select_menu(|m| {
            m.custom_id(custom_id);
            m.options(|o| {
                for (label, value) in &options {
                    o.create_option(|oo| {
                        oo.label(&label.to_string());
                        oo.value(&value.to_string())
                    });
                }
                o
            })
        })
    })
}

pub async fn handle_interaction(ctx: Context, interaction: Interaction) {
    match interaction {
        Interaction::ApplicationCommand(command) => {
            command
                .create_interaction_response(&ctx.http, |response| {
                    response.kind(InteractionResponseType::DeferredChannelMessageWithSource)
                })
                .await
                .unwrap();

            let content = match command.data.name.as_str() {
                "vahti" => super::vahti::run(&ctx, &command).await,
                "poistavahti" => super::poistavahti::run(&ctx, &command).await,
                "poistaesto" => super::poistaesto::run(&ctx, &command).await,
                _ => unreachable!(),
            };

            if !content.is_empty() {
                command
                    .edit_original_interaction_response(&ctx.http, |message| {
                        message.content(&content)
                    })
                    .await
                    .unwrap();
            }
        }
        Interaction::MessageComponent(button) => {
            button
                .create_interaction_response(&ctx.http, |response| {
                    response.kind(InteractionResponseType::DeferredChannelMessageWithSource)
                })
                .await
                .unwrap();

            if button.data.custom_id == "remove_vahti" {
                let message = button.message.clone();
                let urls: Vec<_> = message
                    .embeds
                    .iter()
                    .filter_map(|e| e.footer.as_ref().map(|f| f.text.clone()))
                    .unique()
                    .collect();

                if !urls.is_empty() {
                    button
                        .edit_original_interaction_response(&ctx.http, |m| {
                            m.components(|c| {
                                menu_from_options(
                                    c,
                                    "remove_vahti_menu",
                                    urls.iter().zip(urls.iter()).collect::<Vec<_>>(),
                                )
                            })
                        })
                        .await
                        .unwrap();
                } else {
                    button
                        .edit_original_interaction_response(&ctx.http, |m| {
                                    m.content("Creating Vahti deletion menu failed, try deleting the Vahti manually with /poistavahti")
                        })
                    .await.unwrap();
                }
            } else if button.data.custom_id == "block_seller" {
                let message = button.message.clone();

                let urls: Vec<_> = message
                    .embeds
                    .iter()
                    .filter_map(|e| e.footer.as_ref().map(|f| f.text.clone()))
                    .collect();

                assert!(!urls.is_empty(), "Cannot determine search url");

                // FIXME: We can get the names from the fields also
                let sellers = message
                    .embeds
                    .iter()
                    .map(|e| e.fields.iter().find(|f| f.name == "Myyjä"))
                    .filter_map(|f| f.map(|ff| ff.value.clone()))
                    .filter_map(|s| match s {
                        _ if s.contains("https://www.tori.fi/li?&aid=") => Some((
                            s[1..s.find(']').unwrap()].to_string(),
                            format!(
                                "{},{}",
                                &s[s.rfind('=').unwrap() + 1..s.find(')').unwrap()],
                                crate::tori::ID
                            ),
                        )),
                        _ if s.contains("https://www.huuto.net/kayttaja/") => Some((
                            s[1..s.find(']').unwrap()].to_string(),
                            format!(
                                "{},{}",
                                &s[s.rfind('/').unwrap() + 1..s.find(')').unwrap()],
                                crate::huutonet::ID
                            ),
                        )),
                        _ => None,
                    })
                    .unique()
                    .collect::<Vec<_>>();

                button
                    .edit_original_interaction_response(&ctx.http, |m| {
                        m.content("Choose the seller to block");
                        m.components(|c| menu_from_options(c, "block_seller_menu", sellers))
                    })
                    .await
                    .unwrap();
            } else if button.data.custom_id == "unblock_seller" {
                let db = ctx.get_db().await.unwrap();
                let userid = button.user.id.0;
                let ids: Vec<&str> = button.data.values[0].split(',').collect();
                let sellerid = ids[0].parse::<i32>().unwrap();
                let siteid = ids[1].parse::<i32>().unwrap();

                db.remove_seller_from_blacklist(userid.try_into().unwrap(), sellerid, siteid)
                    .await
                    .unwrap();
                button
                    .edit_original_interaction_response(&ctx.http, |m| m.content("Esto poistettu!"))
                    .await
                    .unwrap();
            } else if button.data.custom_id == "remove_vahti_menu" {
                let userid = button.user.id.0;
                let url = button.data.values[0].to_string();
                let db = ctx.get_db().await.unwrap();

                crate::vahti::remove_vahti(db, &url, userid).await.unwrap();
                button
                    .edit_original_interaction_response(&ctx.http, |m| {
                        m.content(format!("Poistettu vahti: `{}`", url))
                    })
                    .await
                    .unwrap();
            } else if button.data.custom_id == "block_seller_menu" {
                let db = ctx.get_db().await.unwrap();
                let userid = button.user.id.0;
                let ids: Vec<&str> = button.data.values[0].split(',').collect();
                let sellerid = ids[0].parse::<i32>().unwrap();
                let siteid = ids[1].parse::<i32>().unwrap();

                db.add_seller_to_blacklist(userid as i64, sellerid, siteid)
                    .await
                    .unwrap();
                button
                    .edit_original_interaction_response(&ctx.http, |m| m.content("Myyjä estetty!"))
                    .await
                    .unwrap();
            }
        }
        _ => {}
    }
}
