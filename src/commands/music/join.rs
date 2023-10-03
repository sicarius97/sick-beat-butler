use std::{
    sync::{Arc, atomic::AtomicUsize},
    time::Duration,
};

use crate::utils::check_msg;

use serenity::{
    client::Context,
    framework::standard::{
            macros::command,
            CommandResult,
        },
    prelude::Mentionable,
    model::channel::Message,
};

use songbird::{
    Event,
    TrackEvent,
};

use super::handlers::*;

#[command]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        },
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let (handle_lock, success) = manager.join(guild_id, connect_to).await;

    if let Ok(_channel) = success {
        check_msg(
            msg.channel_id
                .say(&ctx.http, &format!("Joined {}", connect_to.mention()))
                .await,
        );
        let mut handler = handle_lock.lock().await;

        let send_http = ctx.http.clone();

        handler .remove_all_global_events();

        handler.add_global_event(
            Event::Track(TrackEvent::Play),
            TrackPlayNotifier {
                chan_id: msg.channel_id,
                http: send_http,
            },
        );

        handler.add_global_event(
            Event::Periodic(Duration::from_secs(1), None),
            IdleHandler {
                http: ctx.http.clone(),
                manager: songbird::get(ctx).await.unwrap().clone(),
                chan_id: msg.channel_id,
                guild_id: guild_id.into(),
                limit: 60 * 10,
                count: Arc::new(AtomicUsize::new(0)),
            }
        );

    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Error joining the channel")
                .await,
        );
    }

    Ok(())
}
