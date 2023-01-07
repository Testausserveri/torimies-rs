mod blacklist;
pub mod database;
mod discord;
pub mod extensions;
mod huutonet;
mod interaction;
mod itemhistory;
pub mod models;
mod notifications;
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

use clokwerk::{Scheduler, TimeUnits};
use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::*;
use serenity::http::Http;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::Interaction;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tracing::{error, info};

use crate::database::Database;
use crate::extensions::ClientContextExt;
use crate::interaction::handle_interaction;
use crate::itemhistory::ItemHistory;
use crate::owner::*;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        handle_interaction(ctx, interaction).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);

        let _ = Command::create_global_application_command(&ctx.http, |command| {
            discord::commands::vahti::register(command);
            discord::commands::poistavahti::register(command);
            discord::commands::poistaesto::register(command)
        })
        .await;
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

    tracing_subscriber::fmt::init();

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

    let http = Http::new(&token);

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

    let mut client = Client::builder(&token, GatewayIntents::non_privileged())
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

    let database = client.get_db().await.unwrap();
    let itemhistory = client.get_itemhistory().await.unwrap();

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
