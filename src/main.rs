mod blacklist;
pub mod database;
pub mod extensions;
mod itemhistory;
pub mod models;
mod owner;
pub mod schema;
mod tori;
mod vahti;

#[macro_use]
extern crate tracing;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate diesel;

use std::collections::HashSet;
use std::env;
use std::sync::Arc;

use blacklist::blacklist_seller;
use clokwerk::{Scheduler, TimeUnits};
use database::Database;
use itemhistory::ItemHistory;
use owner::*;
use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::*;
use serenity::http::Http;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::model::interactions::application_command::{
    ApplicationCommand, ApplicationCommandOptionType,
};
use serenity::model::interactions::{Interaction, InteractionResponseType};
use serenity::prelude::*;
use tori::seller::get_seller_name_from_id;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use vahti::{is_valid_url, new_vahti, remove_vahti};

use crate::extensions::ClientContextExt;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
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
                        .parse::<i32>()
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
                    let sellerid = button.data.values[0].parse::<i32>().unwrap();
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

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
        ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command
                        .name("vahti")
                        .description("Luo uusi vahti")
                        .create_option(|option| {
                            option
                                .name("url")
                                .description("Hakulinkki")
                                .required(true)
                                .kind(ApplicationCommandOptionType::String)
                        })
                })
                .create_application_command(|command| {
                    command
                        .name("poistavahti")
                        .description("Poista olemassaoleva vahti")
                        .create_option(|option| {
                            option
                                .name("url")
                                .description("Hakulinkki")
                                .required(true)
                                .kind(ApplicationCommandOptionType::String)
                        })
                })
                .create_application_command(|command| {
                    command
                        .name("poistaesto")
                        .description("Salli aiemmin estetty myyjä")
                })
        })
        .await
        .unwrap();
    }
    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(update_all_vahtis)]
struct General;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    FmtSubscriber::builder()
        .with_env_filter(EnvFilter::new("info,sqlx::query=error"))
        .init();

    let database = Database::new().await;
    let itemhistory = ItemHistory::new();

    let token = env::var("DISCORD_TOKEN").expect("Expected token in the environment");

    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Expected application-id in the environment")
        .parse()
        .expect("Application id is invalid");

    let update_interval: u32 = env::var("UPDATE_INTERVAL")
        .unwrap_or_else(|_| "60".to_string()) // Default to 1 minute
        .parse()
        .expect("Update interval is invalid");

    let http = Http::new_with_token(&token);

    let (owner, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owner).prefix("!"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .application_id(application_id)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error while creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<Database>(database);
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<ItemHistory>(Arc::new(Mutex::new(itemhistory)));
    }

    let shard_manager = client.shard_manager.clone();

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let mut scheduler = Scheduler::with_tz(chrono::Local);

    let http = client.cache_and_http.http.clone();
    let data = client.data.clone();

    let database = client.get_db().await.unwrap();
    let mut itemhistory = data.write().await.get_mut::<ItemHistory>().unwrap().clone();

    scheduler.every(update_interval.second()).run(move || {
        if let Err(e) = runtime.block_on(vahti::update_all_vahtis(
            database.clone(),
            itemhistory.clone(),
            http.clone(),
        )) {
            error!("Failed to update vahtis: {}", e);
        }
    });

    let thread_handle = scheduler.watch_thread(std::time::Duration::from_millis(1000));

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl-c handler");
        thread_handle.stop();
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
