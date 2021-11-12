mod tori;
mod vahti;
mod owner;
pub mod database;

use std::{collections::HashSet, env, sync::Arc};

use owner::*;
use vahti::new_vahti;
use serenity::{
    async_trait,
    framework::standard::*,
    framework::standard::macros::group,
    model::{
        event::ResumedEvent,
        gateway::Ready,
        interactions::{
            application_command::{
                ApplicationCommand,
                ApplicationCommandOptionType,
            },
            Interaction,
            InteractionResponseType,
        },
    },
    client::bridge::gateway::ShardManager,
    prelude::*,
    http::Http,
};
use tracing::{error, info};

struct Database {
    database: sqlx::SqlitePool,
}

impl Database {
    pub async fn new() -> Database {
        let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("database.sqlite")
                .create_if_missing(true),
        )
        .await
        .expect("Couldn't connect to database");
        sqlx::migrate!("./migrations").run(&database).await.expect("Couldn't run database migrations");
        Self { database }
    }
}

pub struct ShardManagerContainer;

impl TypeMapKey for Database {
    type Value = Arc<Database>;
}

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
                            _ => unreachable!()
                        }
                    }
                    new_vahti(&ctx, &url, command.user.id.0).await
                },
                _ => {
                    unreachable!();
                }
            };
            command.create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                }).await.unwrap()
        };
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
        ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("vahti").description("Luo uusi vahti")
                        .create_option(|option| {
                            option.name("url")
                                .description("Hakulinkki")
                                .required(true)
                                .kind(ApplicationCommandOptionType::String)
                        })
                })
        }).await.unwrap();
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

    let framework = StandardFramework::new().configure(|c| c.owners(owner).prefix("!"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .application_id(application_id)
        .framework(framework)
        .event_handler(Handler)
        .await.expect("Error while creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<Database>(Arc::new(database));
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl-c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
