//! Requires the 'framework' feature flag be enabled for serenity in your project's
//! `Cargo.toml`.

mod commands;

use std::{
    env,
    sync::{
        Arc,
    },
};

use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    framework::{
        standard::{
            macros::{group},
        },
        StandardFramework,
    },
    model::{gateway::Ready},
};

use songbird::{
    SerenityInit,
};
use serenity::client::bridge::gateway::ShardManager;
use serenity::model::event::ResumedEvent;
use serenity::prelude::*;
use tracing::{error, info};

use crate::commands::owner::*;
use crate::commands::music::*;
use crate::commands::stonks::*;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(
    join, bust, stock, mute, unmute, deafen, undeafen, stop, leave, queue, ping, skip, quit
)]
struct General;

#[tokio::main]
async fn main() {
    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    dotenv::dotenv().expect("Failed to load .env file");

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to `debug`.
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Create the framework
    let framework =
        StandardFramework::new().configure(|c| c.prefix("$")).group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .register_songbird()
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}