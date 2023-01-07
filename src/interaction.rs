use serenity::model::application::component::ActionRowComponent;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::prelude::*;

use crate::blacklist::blacklist_seller;
use crate::extensions::ClientContextExt;
use crate::vahti::remove_vahti;

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
                "vahti" => crate::discord::commands::vahti::run(&ctx, &command).await,
                "poistavahti" => crate::discord::commands::poistavahti::run(&ctx, &command).await,
                "poistaesto" => crate::discord::commands::poistaesto::run(&ctx, &command).await,
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
            if button.data.custom_id == "remove_vahti" {
                let userid = button.user.id.0;
                let message = button.message.clone();
                let mut url = String::from("");
                message.components[0]
                    .components
                    .iter()
                    .find(|b| {
                        if let ActionRowComponent::Button(bb) = b {
                            if bb.label.as_ref().unwrap() == "Avaa Hakusivu" {
                                url = bb.url.as_ref().unwrap().clone();
                                return true;
                            }
                            false
                        } else {
                            false
                        }
                    })
                    .unwrap();
                let response = if url.is_empty() {
                    error!("No search url in button, not deleting vahti");
                    String::from("Virhe tapahtui vahdin poistossa")
                } else {
                    remove_vahti(&ctx, &url, userid).await.unwrap()
                };
                button
                    .create_interaction_response(&ctx.http, |r| {
                        r.kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|m| m.content(response))
                    })
                    .await
                    .unwrap()
            } else if button.data.custom_id == "block_seller" {
                let userid = button.user.id.0;
                let message = button.message.clone();
                let embed = &message.embeds[0];
                let mut url = String::new();
                message.components[0]
                    .components
                    .iter()
                    .find(|b| {
                        if let ActionRowComponent::Button(bb) = b {
                            if bb.label.as_ref().unwrap() == "Avaa Hakusivu" {
                                url = bb.url.as_ref().unwrap().clone();
                                return true;
                            }
                            false
                        } else {
                            false
                        }
                    })
                    .unwrap();
                if url.is_empty() {
                    panic!("Cannot determine search url");
                }
                let seller_string = &embed
                    .fields
                    .iter()
                    .find(|f| f.name == "Myyjä")
                    .unwrap()
                    .value;

                let sellerid: i32 = match crate::vahti::SiteId::from(url.as_str()) {
                    crate::vahti::SiteId::Tori => seller_string
                        [seller_string.rfind('=').unwrap() + 1..seller_string.find(')').unwrap()]
                        .parse()
                        .unwrap(),
                    crate::vahti::SiteId::Huutonet => seller_string
                        [seller_string.rfind('/').unwrap() + 1..seller_string.find(')').unwrap()]
                        .parse()
                        .unwrap(),
                    _ => panic!("Cannot block seller from unknown site"),
                };
                let response = blacklist_seller(
                    &ctx,
                    userid,
                    sellerid,
                    crate::vahti::SiteId::from(url.as_str()) as i32,
                )
                .await
                .unwrap();
                button
                    .create_interaction_response(&ctx.http, |r| {
                        r.kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|m| m.content(response))
                    })
                    .await
                    .unwrap()
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
                    .create_interaction_response(&ctx.http, |r| {
                        r.kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|m| m.content("Esto poistettu!"))
                    })
                    .await
                    .unwrap()
            } else if button.data.custom_id == "remove_vahti_menu" {
                let userid = button.user.id.0;
                let url = button.data.values[0].to_string();
                crate::vahti::remove_vahti(&ctx, &url, userid)
                    .await
                    .unwrap();
                button
                    .create_interaction_response(&ctx.http, |r| {
                        r.kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|m| {
                                m.content(format!("Poistettu vahti: `{}`", url))
                            })
                    })
                    .await
                    .unwrap()
            }
        }
        _ => {}
    }
}
