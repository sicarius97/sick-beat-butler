use songbird::{
    Event,
    EventContext,
    EventHandler as VoiceEventHandler, driver::opus::ffi::OPUS_GET_FORCE_CHANNELS_REQUEST
};

use std::sync::Arc;
use crate::utils::check_msg;

use serenity::{
    async_trait,
    http::Http,
    model::prelude::{ChannelId, Message},
};

use super::embeds::{EmbedType, generate_embed};

pub struct TrackPlayNotifier {
    pub chan_id: ChannelId,
    pub http: Arc<Http>,
    pub msg: Message,
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
