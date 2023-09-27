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
use crate::commands::music::check_msg;
use reqwest::header::{HeaderValue, USER_AGENT, CONTENT_TYPE, CONTENT_LENGTH, HeaderMap};
use tracing::info;

#[derive(Debug, Deserialize)]
struct TickerResult {
    o: f64,
    c: f64,
}

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(CONTENT_LENGTH, HeaderValue::from_static("0"));
    headers
}

fn percentage_change(open: f64, close: f64) -> (String, f64) {
    if open > close {
        let decrease = open - close;
        let pct_decrease = decrease / open * 100.0;

        return ("decrease".into(), round_two(pct_decrease))
    } else {
        let increase = close - open;
        let pct_increase = increase / open * 100.0;
        return ("increase".into(), round_two(pct_increase))
    }
}

fn round_two(num: f64) -> f64 {
    (num * 100.0).round() / 100.0
}


#[command]
#[only_in(guilds)]
async fn stock(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let ticker = match args.single::<String>() {
        Ok(ticker) => ticker,
        Err(_) => {
            check_msg(
            msg.channel_id
                .say(&ctx.http, "Must provide a URL to a video or audio")
                .await,
            );
            

            return Ok(());
        },
    };
    let now = Local::now();

    println!("{}-{}-{}", now.year().to_string(), now.month().to_string(), now.day().to_string());

    let yesterday: DateTime<Local> = now.checked_sub_signed(Duration::days(1)).unwrap();

    let yesterday_string: String = format!("{}-{}-{}", yesterday.year().to_string(), yesterday.month().to_string(), yesterday.day().to_string());

    info!("{}", &yesterday_string);

    let query_url: String = format!("https://finnhub.io/api/v1/quote?symbol={}&token={}", ticker.to_ascii_uppercase(), env::var("FINNHUB_TOKEN")?).into();

    let client = reqwest::Client::new();
    let ticker_data = client
        .get(query_url)
        .headers(construct_headers())
        .send()
        .await?.json::<TickerResult>().await.unwrap();

    let change = percentage_change(ticker_data.o.clone(), ticker_data.c.clone());

    let ticker_message = format!("On {} the ticker {} had an open of ${} and a close of ${}\nfor a {} of {}%", yesterday_string, ticker.to_ascii_uppercase(), ticker_data.o, ticker_data.c, change.0, change.1);

    
    check_msg(
    msg.channel_id
        .say(&ctx.http, &ticker_message)
        .await,
    );

    Ok(())
}
