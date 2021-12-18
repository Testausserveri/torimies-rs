use serenity::model::interactions::{Interaction, InteractionResponseType};
use serenity::prelude::*;

use crate::blacklist::blacklist_seller;
use crate::extensions::ClientContextExt;
use crate::tori::seller::get_seller_name_from_id;
use crate::vahti::{is_valid_url, new_vahti, remove_vahti};

pub async fn handle_interaction(ctx: Context, interaction: Interaction) {
    match interaction {
        Interaction::ApplicationCommand(command) => {
            let content = match command.data.name.as_str() {
                "vahti" => {
                    let mut url: String = "".to_string();
                    for a in &command.data.options {
                        match a.name.as_str() {
                            "url" => {
                                let tempurl = a.value.as_ref().unwrap();
                                url = tempurl.as_str().unwrap().to_string();
                            }
                            _ => unreachable!(),
                        }
                    }
                    if !is_valid_url(&url).await {
                        "Annettu hakuosoite on virheellinen tai kyseiselle haulle ei ole tällä hetkellä tuloksia! Vahtia ei luoda.".to_string()
                    } else {
                        new_vahti(&ctx, &url, command.user.id.0).await.unwrap()
                    }
                }
                "poistavahti" => {
                    let mut url: String = "".to_string();
                    for a in &command.data.options {
                        match a.name.as_str() {
                            "url" => {
                                let tempurl = a.value.as_ref().unwrap();
                                url = tempurl.as_str().unwrap().to_string();
                            }
                            _ => unreachable!(),
                        }
                    }
                    remove_vahti(&ctx, &url, command.user.id.0).await.unwrap()
                }
                "poistaesto" => String::from("Valitse poistettava(t) esto(t)"),
                _ => {
                    unreachable!();
                }
            };
            let db = ctx.get_db().await.unwrap();
            let blacklist = db
                .fetch_user_blacklist(command.user.id.0.try_into().unwrap())
                .await
                .unwrap();
            let mut blacklist_names = Vec::new();
            for entry in &blacklist {
                blacklist_names.push(
                    get_seller_name_from_id(*entry)
                        .await
                        .unwrap_or("Unknown Seller".to_string()),
                );
            }
            command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.content(&content);
                            if content == *"Valitse poistettava(t) esto(t)" {
                                message.components(|c| {
                                    c.create_action_row(|r| {
                                        r.create_select_menu(|m| {
                                            m.custom_id("unblock_seller");
                                            m.options(|o| {
                                                for (i, id) in blacklist.iter().enumerate() {
                                                    o.create_option(|oo| {
                                                        oo.label(blacklist_names[i].clone());
                                                        oo.value(id)
                                                    });
                                                }
                                                o
                                            })
                                        })
                                    })
                                });
                            };
                            message
                        })
                })
                .await
                .unwrap()
        }
        Interaction::MessageComponent(button) => {
            if button.data.custom_id == "remove_vahti" {
                let userid = button.user.id.0;
                let embed = button.message.clone().regular().unwrap();
                let embed = embed.embeds[0].description.as_ref().unwrap();
                let url = &embed[embed.rfind('(').unwrap() + 1..embed.rfind(')').unwrap()];
                let response = remove_vahti(&ctx, url, userid).await.unwrap();
                button
                    .create_interaction_response(&ctx.http, |r| {
                        r.kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|m| m.content(response))
                    })
                    .await
                    .unwrap()
            } else if button.data.custom_id == "block_seller" {
                let userid = button.user.id.0;
                let message = button.message.clone().regular().unwrap();
                let embed = &message.embeds[0];
                let seller_string = &embed
                    .fields
                    .iter()
                    .find(|f| f.name == "Myyjä")
                    .unwrap()
                    .value;
                let sellerid = seller_string
                    [seller_string.rfind('=').unwrap() + 1..seller_string.find(')').unwrap()]
                    .parse::<i64>()
                    .unwrap();
                let response = blacklist_seller(&ctx, userid, sellerid).await.unwrap();
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
                let sellerid = button.data.values[0].parse::<i64>().unwrap();
                db.remove_seller_from_blacklist(userid.try_into().unwrap(), sellerid)
                    .await
                    .unwrap();
                button
                    .create_interaction_response(&ctx.http, |r| {
                        r.kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|m| m.content("Esto poistettu!"))
                    })
                    .await
                    .unwrap()
            }
        }
        _ => {}
    }
}
