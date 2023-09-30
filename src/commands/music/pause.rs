use serenity::{
    client::Context,
    framework::standard::{
            macros::command,
            CommandResult,
        },
    model::prelude::Message,
};

use crate::error::ButlerError;

#[command]
pub async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();

    let _ = songbird::get(ctx)
        .await
        .ok_or_else(|| ButlerError::log("Couldn't get songbird"))?
        .get(guild.id)
        .ok_or_else(|| ButlerError::log("No Call"))?
        .lock()
        .await
        .queue()
        .current()
        .ok_or_else(|| ButlerError::user("No track playing"))?
        .pause()
        .map_err(|e| {
            ButlerError::user_and_log(
                "Failed to pause :person_shrugging:",
                format!("Failed to pause: {}", e).as_str(),
            )
        });

    Ok(())
}