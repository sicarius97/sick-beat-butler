use songbird::{
    Event,
    EventContext,
    EventHandler as VoiceEventHandler, Songbird, tracks::PlayMode,
    id::GuildId,
};

use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use crate::utils::check_msg;

use serenity::{
    async_trait,
    http::Http,
    model::prelude::ChannelId,
};

use super::embeds::{EmbedType, generate_embed};

pub struct TrackPlayNotifier {
    pub chan_id: ChannelId,
    pub http: Arc<Http>,
}

#[async_trait]
impl VoiceEventHandler for TrackPlayNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            let (_state, handle) = track_list.first().unwrap();
            
            let e = generate_embed(EmbedType::NowPlaying(handle.metadata().clone()));

            let _ = check_msg(self.chan_id.send_message(&self.http, |m| m.set_embed(e)).await);
        }

        None
    }
}

pub struct IdleHandler {
    pub http: Arc<Http>,
    pub manager: Arc<Songbird>,
    pub chan_id: ChannelId,
    pub guild_id: GuildId,
    pub limit: usize,
    pub count: Arc<AtomicUsize>,
}

#[async_trait]
impl VoiceEventHandler for IdleHandler {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        let EventContext::Track(track_list) = ctx else {
            return None;
        };

        // looks like the track list isn't ordered here, so the first track in the list isn't
        // guaranteed to be the first track in the actual queue, so search the entire list
        let bot_is_playing = track_list
            .iter()
            .any(|track| matches!(track.0.playing, PlayMode::Play));

        // if there's a track playing, then reset the counter
        if bot_is_playing {
            self.count.store(0, Ordering::Relaxed);
            return None;
        }

        if self.count.fetch_add(1, Ordering::Relaxed) >= self.limit {
            if self.manager.remove(self.guild_id).await.is_ok() {
                self.chan_id
                    .say(&self.http, "The musical vibes have ceased so I must leave")
                    .await
                    .unwrap();
            }
        }

        None
    }
}
