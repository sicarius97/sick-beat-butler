mod embeds;
pub mod queue;
pub mod pause;
pub mod resume;
pub mod join;
mod handlers;

use songbird::input::Metadata;
use std::time::Duration;

/// `split_duration` splits a [`Duration`] into a (minutes, seconds) tuple
const fn split_duration(d: Duration) -> (u64, u64) {
    (d.as_secs() / 60, d.as_secs() % 60)
}

fn get_title(m: &Metadata) -> &str {
    m.track
        .as_deref()
        .or_else(|| m.title.as_deref())
        .unwrap_or("Unknown Title")
}

fn get_artist(m: &Metadata) -> &str {
    m.artist
        .as_deref()
        .or_else(|| m.channel.as_deref())
        .unwrap_or("Unknown Artist")
}

pub fn get_song(m: &Metadata) -> String {
    format!("{} by {}", get_title(m), get_artist(m))
}