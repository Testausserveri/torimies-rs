use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

use crate::extensions::ClientContextExt;
use crate::vahti::remove_vahti;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("poistavahti")
        .description("Poista olemassaoleva vahti")
        .create_option(|option| {
            option
                .name("url")
                .description("Hakusivun linkki")
                .kind(CommandOptionType::String)
        })
}

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> String {
    let mut url = String::new();
    for a in &command.data.options {
        match a.name.as_str() {
            "url" => url = String::from(a.value.as_ref().unwrap().as_str().unwrap()),
            _ => unreachable!(),
        }
    }

    if !url.is_empty() {
        remove_vahti(ctx, &url, command.user.id.0).await.unwrap()
    } else {
        let db = ctx.get_db().await.unwrap();
        let vahtilist = db
            .fetch_vahti_entries_by_user_id(command.user.id.0 as i64)
            .await
            .unwrap();

        command
            .edit_original_interaction_response(&ctx.http, |message| {
                message.content("Valitse poistettava(t) vahti/vahdit");

                if vahtilist.is_empty() {
                    message.content("Ei vahteja! Aseta vahti komennolla `/vahti`")
                } else {
                    message.components(|c| {
                        c.create_action_row(|r| {
                            r.create_select_menu(|m| {
                                m.custom_id("remove_vahti_menu");
                                m.options(|o| {
                                    for vahti in &vahtilist {
                                        o.create_option(|oo| {
                                            oo.label(&vahti.url);
                                            oo.value(&vahti.url)
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
}
