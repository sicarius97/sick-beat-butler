use std::{time::Duration, sync::Arc};

use serenity::{
    builder::CreateActionRow,
    client::Context,
    futures::prelude::*,
    model::{
        channel::Message,
        id::{ChannelId, GuildId},
        interactions::{message_component::ButtonStyle, InteractionResponseType},
    }, http::Http,

};
use songbird::{tracks::TrackHandle, input::Metadata, EventContext};
use super::*;

#[derive(Clone)]
pub enum EmbedType {
    Queue(Vec<TrackHandle>),
    NowPlaying(Metadata),
}

pub fn generate_embed(ctx: EmbedType) -> serenity::builder::CreateEmbed {
    let mut embed = serenity::builder::CreateEmbed::default();
    match ctx {
        EmbedType::Queue(queue) => {
            if let Some(track) = queue.first() {
                let current = track.metadata();
                embed.author(|a| a.name("Now Playing:"));
                embed.title(get_song(current));
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
        EmbedType::NowPlaying(metadata) => {
            let playing = &metadata;
            embed.author(|a| a.name("Now Playing:"));
            embed.title(get_song(playing));
            embed.color(0x00ff00);
            embed.field("Duration", format!("{}:{}", playing.duration.unwrap_or_default().as_secs() / 60, playing.duration.unwrap_or_default().as_secs() % 60), false);

            if let Some(thumbnail) = &playing.thumbnail {
                embed.thumbnail(thumbnail);
            }
        
            if let Some(url) = &playing.source_url {
                embed.url(url);
            }
        }
    }

    embed
}

// Sends generated embed to chat based on its type
pub async fn send_embed(http: &Arc<Http>, msg: &Message, embed: EmbedType) -> serenity::Result<()> {
    let e = generate_embed(embed.clone());
    let _ = match embed {
        EmbedType::Queue(_) => msg
            .channel_id
            .send_message(&http, |m| m.set_embed(e))
            .await,
        EmbedType::NowPlaying(_) => msg
            .channel_id
            .send_message(&http, |m| m.set_embed(e))
            .await,
    };

    Ok(())
}

