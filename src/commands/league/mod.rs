use serenity::{
    client::Context,
    framework::standard::{
            macros::command,
            Args,
            CommandResult,
        },
    model::channel::Message,
};
use chrono::prelude::*;
use chrono::Duration;
use serde::Deserialize;
use std::env;
use crate::utils::check_msg;
use reqwest::header::{HeaderValue, USER_AGENT, CONTENT_TYPE, CONTENT_LENGTH, HeaderMap};
use tracing::{error, info};
use crate::types::ButlerResult;

#[derive(Debug, Deserialize)]
struct SummonerResult {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
struct LeagueResult {
    league_result: Vec<LeagueData>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LeagueData {
    summoner_name: String,
    queue_type: String,
    tier: String,
    rank: String,
    league_points: u32,
    wins: u32,
    losses: u32,
}

// Fetches league of legends summoner data from Riot API by username
async fn get_summoner_data(name: &str) -> Option<SummonerResult> {
    let api_key = match std::env::var("RIOT_TOKEN") {
        Ok(key) => key,
        Err(_) => {
            error!("No Riot API key found");
            return None;
        },
    };
    let url = format!("https://na1.api.riotgames.com/lol/summoner/v4/summoners/by-name/{}?api_key={}", name, api_key);

    info!("Fetching summoner data from: {}", url);

    let _ = match reqwest::get(&url).await {
        Ok(res) => {
            match res.json::<SummonerResult>().await {
                Ok(data) => {
                    return Some(data);
                },
                Err(e) => {
                    error!("Error parsing summoner JSON: {}", e);
                    return None;
                },
            }
        },
        Err(e) => {
            error!("Error fetching summoner data: {}", e);
            return None;
        },
    };
}

// Fetches further league of legends summoner data from Riot API by summoner id
async fn get_league_data(id: &str) -> Option<LeagueResult> {
    let api_key = match std::env::var("RIOT_TOKEN") {
        Ok(key) => key,
        Err(_) => {
            error!("No Riot API key found");
            return None;
        },
    };
    let url = format!("https://na1.api.riotgames.com/lol/league/v4/entries/by-summoner/{}?api_key={}", id, api_key);

    let _ = match reqwest::get(&url).await {
        Ok(res) => {
            match res.json::<LeagueResult>().await {
                Ok(data) => {
                    return Some(data);
                },
                Err(e) => {
                    error!("Error parsing league JSON: {}", e);
                    return None;
                },
            }
        },
        Err(e) => {
            error!("Error fetching league data: {}", e);
            return None;
        },
    };
}

#[command]
#[only_in(guilds)]
async fn league(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let summoner = match args.single::<String>() {
        Ok(summoner) => summoner,
        Err(_) => {
            check_msg(
            msg.channel_id
                .say(&ctx.http, "Must provide a summoner value")
                .await,
            );
            

            return Ok(());
        },
    };

    info!("Fetching summoner data: {}", summoner);

    let summoner_data = match get_summoner_data(&summoner).await {
        Some(summoner) => {
            info!("{:?}", summoner);
            summoner
        },
        None => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Summoner not found")
                    .await,
            );

            return Ok(())
        },
    };

    if summoner.to_lowercase() == summoner_data.name.to_lowercase() {
        let _ = match get_league_data(&summoner_data.id).await {
            Some(league) => {
                info!("{:?}", league);
            },
            None => {
                check_msg(
                    msg.channel_id
                        .say(&ctx.http, "League data not found")
                        .await,
                );

                return Ok(())
            },
        };
    }


    check_msg(
        msg.channel_id
            .say(&ctx.http, "Test successful")
            .await,
    );

    Ok(())
}
