mod extensions;
mod interaction;
mod poistaesto;
mod poistavahti;
mod vahti;

use async_trait::async_trait;
use serenity::model::application::command;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::GatewayIntents;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::command::Command;
use crate::database::Database;
use crate::error::Error;

pub const NAME: &str = "discord";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        interaction::handle_interaction(ctx, interaction).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);

        let _ = command::Command::create_global_application_command(&ctx.http, |command| {
            vahti::register(command);
            poistavahti::register(command);
            poistaesto::register(command)
        })
        .await;
    }
    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

pub struct Discord {
    pub client: Client,
}

impl Discord {
    pub async fn init(db: &Database) -> Result<Self, Error> {
        let token =
            std::env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in the environment");

        let application_id: u64 = std::env::var("APPLICATION_ID")
            .expect("Expected APPLICATION_ID in the environment")
            .parse()
            .expect("Invalid APPLICATION_ID");

        let client = Client::builder(&token, GatewayIntents::non_privileged())
            .application_id(application_id)
            .event_handler(Handler)
            .await?;

        let mut data = client.data.write().await;
        data.insert::<Database>(db.to_owned());
        drop(data);

        Ok(Self { client })
    }
}

#[async_trait]
impl Command for Discord {
    async fn start(&mut self) -> Result<(), Error> {
        Ok(self.client.start().await?)
    }

    async fn destroy(&mut self) {
        self.client.shard_manager.lock().await.shutdown_all().await;
    }
}
