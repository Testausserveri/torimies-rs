use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

use crate::extensions::ClientContextExt;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("poistaesto")
        .description("Salli aiemmin estetty myyjä")
}

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> String {
    let db = ctx.get_db().await.unwrap();
    let blacklist = db
        .fetch_user_blacklist(command.user.id.0 as i64)
        .await
        .unwrap();

    let mut blacklist_names = Vec::new();
    for entry in &blacklist {
        blacklist_names.push(match entry.1 {
            1 => crate::tori::seller::get_seller_name_from_id(entry.0)
                .await
                .unwrap_or(String::from("Unknown Seller")),
            2 => crate::huutonet::seller::get_seller_name_from_id(entry.0)
                .await
                .unwrap_or(String::from("Unknown Seller")),
            _ => String::from("Unknown Seller"),
        });
    }

    command
        .edit_original_interaction_response(&ctx.http, |message| {
            message.content("Valitse poistettava(t) esto/estot");

            if blacklist.is_empty() {
                message.content("Ei estettyjä myyjiä!")
            } else {
                message.components(|c| {
                    c.create_action_row(|r| {
                        r.create_select_menu(|m| {
                            m.custom_id("unblock_seller");
                            m.options(|o| {
                                for (i, ids) in blacklist.iter().enumerate() {
                                    o.create_option(|oo| {
                                        oo.label(blacklist_names[i].clone());
                                        oo.value(format!("{},{}", ids.0, ids.1))
                                    });
                                }
                                o
                            })
                        })
                    })
                })
            }
        })
        .await
        .unwrap();

    String::new()
}
