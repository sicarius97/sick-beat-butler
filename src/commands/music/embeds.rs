use std::time::Duration;

use serenity::{
    builder::CreateActionRow,
    client::Context,
    futures::prelude::*,
    model::{
        channel::Message,
        id::{ChannelId, GuildId},
        interactions::{message_component::ButtonStyle, InteractionResponseType},
    },
};
use songbird::{tracks::TrackHandle, input::Metadata};
use super::*;

pub enum EmbedType {
    Queue(Vec<TrackHandle>),
    NowPlaying(TrackHandle, String),
}

fn generate_embed(ctx: EmbedType) -> serenity::builder::CreateEmbed {
    let mut embed = serenity::builder::CreateEmbed::default();
    match ctx {
        EmbedType::Queue(queue) => {
            if let Some(track) = queue.first() {
                let current = track.metadata();
                embed.title("Queue");
                embed.description(get_song(current));
                embed.color(0x00ff00);
                embed.field("Duration", format!("{}:{}", current.duration.unwrap_or_default().as_secs() / 60, current.duration.unwrap_or_default().as_secs() % 60), false);
                embed.field("URL", current.source_url.as_deref().unwrap_or("none"), false);
                /*
                embed.field("Looping", "No", false);
                embed.field("Shuffling", "No", false);
                embed.field("Autoplay", "No", false);
                embed.field("Volume", "100%", false);
                embed.field("Paused", "No", false);
                embed.field("Playing", "No", false);
                embed.field("Stopped", "No", false);
                embed.field("Repeat", "No", false);
                */
            }
        }
        EmbedType::NowPlaying(track, user) => {
            let playing = track.metadata();
            embed.title("Now Playing");
            embed.description(get_song(playing));
            embed.color(0x00ff00);
            embed.field("Duration", format!("{}:{}", playing.duration.unwrap_or_default().as_secs() / 60, playing.duration.unwrap_or_default().as_secs() % 60), false);
            embed.field("URL", playing.source_url.as_deref().unwrap_or("none"), false);
            embed.field("Played by: ", user, false);
        }
    }

    embed
}

