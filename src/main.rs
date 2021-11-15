pub mod database;
pub mod extensions;
mod itemhistory;
mod owner;
mod tori;
mod vahti;

#[macro_use]
extern crate tracing;

use std::{collections::HashSet, env, sync::Arc};

use owner::*;
use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::standard::macros::group,
    framework::standard::*,
    http::Http,
    model::{
        event::ResumedEvent,
        gateway::Ready,
        interactions::{
            application_command::{ApplicationCommand, ApplicationCommandOptionType},
            Interaction, InteractionResponseType,
        },
    },
    prelude::*,
};

use crate::extensions::ClientContextExt;
use database::Database;
use itemhistory::ItemHistory;
use vahti::new_vahti;
use vahti::remove_vahti;

use clokwerk::{Scheduler, TimeUnits};

use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
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
                    new_vahti(&ctx, &url, command.user.id.0).await
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
                    remove_vahti(&ctx, &url, command.user.id.0).await
                }
                _ => {
                    unreachable!();
                }
            };
            command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
                .unwrap()
        };
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
        ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
            commands.create_application_command(|command| {
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

    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    let database = Database::new().await;
    let itemhistory = ItemHistory::new();

    let token = env::var("DISCORD_TOKEN").expect("Expected token in the environment");

    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Expected application-id in the environment")
        .parse()
        .expect("Application id is invalid");

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
        data.insert::<Database>(Arc::new(database));
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<ItemHistory>(Arc::new(Mutex::new(itemhistory)));
    }

    let shard_manager = client.shard_manager.clone();

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let mut scheduler = Scheduler::with_tz(chrono::Local);

    let http = client.cache_and_http.http.clone();
    let data = client.data.clone();

    let database = client.get_db().await;
    let mut itemhistory = data.write().await.get_mut::<ItemHistory>().unwrap().clone();

    scheduler.every(1.minute()).run(move || {
        runtime.block_on(vahti::update_all_vahtis(
            database.to_owned(),
            &mut itemhistory,
            &http,
        ));
    });

    let thread_handle = scheduler.watch_thread(std::time::Duration::from_millis(1000));

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl-c handler");
        shard_manager.lock().await.shutdown_all().await;
        thread_handle.stop();
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
