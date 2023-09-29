use songbird::{
    Event,
    EventContext,
    EventHandler as VoiceEventHandler
};

use std::sync::Arc;
use crate::utils::check_msg;

use serenity::{
    async_trait,
    http::Http,
    model::prelude::ChannelId,
};

use super::get_song;

pub struct TrackPlayNotifier {
    pub chan_id: ChannelId,
    pub http: Arc<Http>,
}
#[async_trait]
impl VoiceEventHandler for TrackPlayNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            let (_, next) = track_list.first().unwrap();

            check_msg(
                self.chan_id
                    .say(&self.http, &format!("Now playing: {}.", get_song(next.metadata())))
                    .await,
            );
        }


        None
    }
}
