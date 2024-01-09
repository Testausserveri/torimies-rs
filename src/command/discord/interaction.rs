use itertools::Itertools;
use serenity::all::ComponentInteractionDataKind;
use serenity::builder::{
    CreateActionRow, CreateSelectMenu, CreateSelectMenuOption, EditInteractionResponse,
};
use serenity::model::application::Interaction;
use serenity::prelude::*;

use super::extensions::ClientContextExt;

pub fn menu_from_options(
    custom_id: &str,
    options: Vec<(impl ToString, impl ToString)>,
) -> Vec<CreateActionRow> {
    let menu_options = options
        .iter()
        .map(|(l, v)| CreateSelectMenuOption::new(l.to_string(), v.to_string()))
        .collect::<Vec<_>>();
    let menu = CreateSelectMenu::new(
        custom_id,
        serenity::builder::CreateSelectMenuKind::String {
            options: menu_options,
        },
    );
    vec![CreateActionRow::SelectMenu(menu)]
}

pub async fn handle_interaction(ctx: Context, interaction: Interaction) {
    match interaction {
        Interaction::Command(command) => {
            command.defer_ephemeral(&ctx.http).await.unwrap();

            let content = match command.data.name.as_str() {
                "vahti" => super::vahti::run(&ctx, &command).await,
                "poistavahti" => super::poistavahti::run(&ctx, &command).await,
                "poistaesto" => super::poistaesto::run(&ctx, &command).await,
                _ => unreachable!(),
            };

            if !content.is_empty() {
                command
                    .edit_response(&ctx.http, EditInteractionResponse::new().content(&content))
                    .await
                    .unwrap();
            }
        }
        Interaction::Component(button) => {
            if button.data.custom_id == "remove_vahti" {
                button.defer_ephemeral(&ctx.http).await.unwrap();
                let message = button.message.clone();
                let urls: Vec<_> = message
                    .embeds
                    .iter()
                    .filter_map(|e| e.footer.as_ref().map(|f| f.text.clone()))
                    .unique()
                    .collect();

                if !urls.is_empty() {
                    button
                        .edit_response(
                            &ctx.http,
                            EditInteractionResponse::new().components(menu_from_options(
                                "remove_vahti_menu",
                                urls.iter().zip(urls.iter()).collect::<Vec<_>>(),
                            )),
                        )
                        .await
                        .unwrap();
                } else {
                    button
                        .edit_response(&ctx.http,
                            EditInteractionResponse::new().content("Creating Vahti deletion menu failed, try deleting the Vahti manually with /poistavahti")
                        )
                    .await.unwrap();
                }
            } else if button.data.custom_id == "block_seller" {
                button.defer_ephemeral(&ctx.http).await.unwrap();
                let message = button.message.clone();

                let urls: Vec<_> = message
                    .embeds
                    .iter()
                    .filter_map(|e| e.footer.as_ref().map(|f| f.text.clone()))
                    .collect();

                assert!(!urls.is_empty(), "Cannot determine search url");

                let sellers = message
                    .embeds
                    .iter()
                    .map(|e| e.fields.iter().find(|f| f.name == "Myyjä"))
                    .filter_map(|f| f.map(|ff| ff.value.clone()))
                    .filter_map(|s| match s {
                        #[cfg(feature = "tori")]
                        _ if s.contains("https://www.tori.fi/li?&aid=") => Some((
                            s[1..s.find(']').unwrap()].to_string(),
                            format!(
                                "{},{}",
                                &s[s.rfind('=').unwrap() + 1..s.find(')').unwrap()],
                                crate::tori::ID
                            ),
                        )),
                        #[cfg(feature = "huutonet")]
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
                    .edit_response(
                        &ctx.http,
                        EditInteractionResponse::new()
                            .content("Choose the seller to block")
                            .components(menu_from_options("block_seller_menu", sellers)),
                    )
                    .await
                    .unwrap();
            } else if button.data.custom_id == "unblock_seller" {
                button.defer_ephemeral(&ctx.http).await.unwrap();
                let db = ctx.get_db().await.unwrap();
                let userid = u64::from(button.user.id);
                let ids: Vec<String> = match button.data.kind.clone() {
                    ComponentInteractionDataKind::StringSelect { values } => {
                        values[0].split(',').map(|s| s.to_string()).collect()
                    }
                    _ => unreachable!(),
                };
                let sellerid = ids[0].parse::<i32>().unwrap();
                let siteid = ids[1].parse::<i32>().unwrap();

                db.remove_seller_from_blacklist(userid.try_into().unwrap(), sellerid, siteid)
                    .await
                    .unwrap();
                button
                    .edit_response(
                        &ctx.http,
                        EditInteractionResponse::new().content("Esto poistettu!"),
                    )
                    .await
                    .unwrap();
            } else if button.data.custom_id == "remove_vahti_menu" {
                button.defer_ephemeral(&ctx.http).await.unwrap();
                let userid = u64::from(button.user.id);
                let url = match button.data.kind.clone() {
                    ComponentInteractionDataKind::StringSelect { values } => values[0].to_string(),
                    _ => unreachable!(),
                };
                let db = ctx.get_db().await.unwrap();

                crate::vahti::remove_vahti(db, &url, userid, crate::delivery::discord::ID)
                    .await
                    .unwrap();
                button
                    .edit_response(
                        &ctx.http,
                        EditInteractionResponse::new()
                            .content(format!("Poistettu vahti: `{}`", url)),
                    )
                    .await
                    .unwrap();
            } else if button.data.custom_id.starts_with("remove_vahti_menu_page_") {
                let page_number: usize = button
                    .data
                    .custom_id
                    .strip_prefix("remove_vahti_menu_page_")
                    .unwrap()
                    .parse()
                    .unwrap();

                button
                    .create_response(
                        &ctx.http,
                        serenity::builder::CreateInteractionResponse::UpdateMessage(
                            super::poistavahti::update_message(
                                &ctx,
                                page_number,
                                u64::from(button.user.id),
                            )
                            .await,
                        ),
                    )
                    .await
                    .unwrap();
                return;
            } else if button.data.custom_id == "block_seller_menu" {
                button.defer_ephemeral(&ctx.http).await.unwrap();
                let db = ctx.get_db().await.unwrap();
                let userid = u64::from(button.user.id);
                let ids: Vec<String> = match button.data.kind.clone() {
                    ComponentInteractionDataKind::StringSelect { values } => {
                        values[0].split(',').map(|s| s.to_string()).collect()
                    }
                    _ => unreachable!(),
                };
                let sellerid = ids[0].parse::<i32>().unwrap();
                let siteid = ids[1].parse::<i32>().unwrap();

                db.add_seller_to_blacklist(userid as i64, sellerid, siteid)
                    .await
                    .unwrap();
                button
                    .edit_response(
                        &ctx.http,
                        EditInteractionResponse::new().content("Myyjä estetty!"),
                    )
                    .await
                    .unwrap();
            }
        }
        _ => {}
    }
}
