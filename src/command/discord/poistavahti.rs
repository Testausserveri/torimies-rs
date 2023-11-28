use serenity::all::ReactionType;
use serenity::builder::{
    CreateActionRow, CreateButton, CreateCommand, CreateCommandOption,
    CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind,
    CreateSelectMenuOption, EditInteractionResponse,
};
use serenity::client::Context;
use serenity::model::application::{CommandInteraction, CommandOptionType};

use super::extensions::ClientContextExt;
use crate::vahti::remove_vahti;

pub fn register() -> CreateCommand {
    CreateCommand::new("poistavahti")
        .description("Poista olemassaoleva vahti")
        .add_option(CreateCommandOption::new(
            CommandOptionType::String,
            "url",
            "Hakusivun linkki",
        ))
}

fn show_select_menu_page(urls: Vec<String>, page: usize) -> Vec<CreateActionRow> {
    let options = urls
        .iter()
        .skip(page * 25)
        .take(25)
        .map(|url| CreateSelectMenuOption::new(url, url))
        .collect::<Vec<_>>();
    let menu = CreateSelectMenu::new(
        "remove_vahti_menu",
        CreateSelectMenuKind::String { options },
    );
    let buttons = vec![
        CreateButton::new(format!(
            "remove_vahti_menu_page_{}",
            if page > 0 { page - 1 } else { 0 }
        ))
        .emoji(ReactionType::Unicode("◀️".to_string()))
        .disabled(page == 0),
        CreateButton::new(format!("remove_vahti_menu_page_{}", page + 1))
            .emoji(ReactionType::Unicode("▶️".to_string()))
            .disabled(page >= urls.len() / 25),
    ];

    vec![
        CreateActionRow::SelectMenu(menu),
        CreateActionRow::Buttons(buttons),
    ]
}

pub async fn update_message(
    ctx: &Context,
    page: usize,
    user_id: u64,
) -> serenity::builder::CreateInteractionResponseMessage {
    let db = ctx.get_db().await.unwrap();

    let vahtilist = db
        .fetch_vahti_entries_by_user_id(user_id as i64)
        .await
        .unwrap();

    let mut urls = vahtilist.iter().cloned().map(|v| v.url).collect::<Vec<_>>();
    urls.sort();

    if vahtilist.is_empty() {
        CreateInteractionResponseMessage::new()
            .content("Ei vahteja! Aseta vahti komennolla `/vahti`")
    } else {
        CreateInteractionResponseMessage::new().components(show_select_menu_page(urls, page))
    }
}

pub async fn run(ctx: &Context, command: &CommandInteraction) -> String {
    let mut url = String::new();
    for a in &command.data.options {
        match a.name.as_str() {
            "url" => url = String::from(a.value.as_str().unwrap()),
            _ => unreachable!(),
        }
    }

    let db = ctx.get_db().await.unwrap();

    if !url.is_empty() {
        remove_vahti(
            db,
            &url,
            u64::from(command.user.id),
            crate::delivery::discord::ID,
        )
        .await
        .unwrap()
    } else {
        let db = ctx.get_db().await.unwrap();
        let vahtilist = db
            .fetch_vahti_entries_by_user_id(u64::from(command.user.id) as i64)
            .await
            .unwrap();

        let urls = vahtilist.iter().cloned().map(|v| v.url).collect::<Vec<_>>();

        let message = if vahtilist.is_empty() {
            EditInteractionResponse::new().content("Ei vahteja! Aseta vahti komennolla `/vahti`")
        } else {
            EditInteractionResponse::new().components(show_select_menu_page(urls, 0))
        };

        command.edit_response(&ctx.http, message).await.unwrap();
        String::new()
    }
}
