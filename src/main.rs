#![warn(clippy::str_to_string)]

mod commands;
mod log {
    use std::{fs::{OpenOptions}, io::Write};
    use ::time::{Error, UtcDateTime, format_description};

    pub fn log_to_file(message: String) -> Result<(), Error>  {
        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("bot.log");
        let timestamp = {
            let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")?;
            let time = UtcDateTime::now();
            let timestamp = time.format(&format)?;
            timestamp
        };
        let log_message = format!("\n{timestamp}: {message}");
        let _ = log_file.unwrap().write_all(&log_message.into_bytes());
        Ok(())
    }
}

use async_std::task;
use poise::{serenity_prelude as serenity};
use rand::Rng;
use ::serenity::{all::{CreateAttachment, CreateMessage, EditMember, EmojiId, ReactionType}, model::{guild::PartialGuild, id::ChannelId}};
use tokio::fs::File;
use std::{fmt::Write, io::Read, sync::Arc, time::Duration};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{fs, fs::File as TokenFile};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug)]
pub struct Data {
    //
}

#[derive(Deserialize, Debug, Serialize)]

pub struct Config {
    lunko_chance: u64,
    mute_list: Vec<serenity::UserId>,
    muted: bool,
    custom_alert_active: bool,
    inactivity_alert_active: bool,
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            let mut error_msg = String::new();
            let _ = write!(&mut error_msg, "Error in command `{}`: {:?}", ctx.command().name, error,);
            let _ = log::log_to_file(error_msg);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                let err_msg = format!("Error while handling error: {}", e);
                let _ = log::log_to_file(err_msg);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let options = poise::FrameworkOptions {
        commands: vec![
            commands::about(), 
            commands::ping(), 
            commands::lunko(), 
            commands::echo(), 
            commands::mutelist(),
            commands::mute(),
            commands::config(),
            commands::ban(),
            commands::whois(),
            commands::whothefuck(),
            commands::scramble(),
            commands::reverse(),
            commands::customalert(),
            commands::inactivityalert(),
            commands::shinx(),
            commands::shinx_collection(),
            commands::unshid_test(),
            ], // COMMANDS
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!".into()),
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(3600),
            ))),
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.author().id == 123456789 {
                    return Ok(false);
                }
                Ok(true)
            })
        }),
        skip_checks_for_owners: false,
        event_handler: |ctx, event, framework, data| {
            Box::pin(event_handler(ctx, event, framework, data))
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", ready.user.name);
                let _ = log::log_to_file("Logged in".to_owned());
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    //
                })
            })
        })
        .options(options)
        .build();
    let mut token = String::new();
    let _ = TokenFile::open("token.txt").unwrap().read_to_string(&mut token);
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::ClientBuilder::new(&token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap()
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot: _ } => {
            loop {
                let rand_duration = rand::thread_rng().gen_range(1..=604800); 
                task::sleep(Duration::from_secs(rand_duration)).await;
                let json_data = fs::read_to_string("config.json")?;
                let current_read: Config = serde_json::from_str(&json_data)?;
                if current_read.muted == false {
                    let channel = {
                        let guild = PartialGuild::get(&ctx.http, 765689692851011595).await.unwrap();
                        let channelid = ChannelId::new(1453439203101900944);
                        let channels = guild.channels(&ctx.http).await.unwrap();
                        let channel = channels[&channelid].clone();
                        channel
                    };
                    channel.say(&ctx.http, "<@&1453436782627520777>").await?;
                }        
            }
        }
        serenity::FullEvent::Message { new_message } => {
                let message_content = new_message.content.to_lowercase();
                let json_data = fs::read_to_string("config.json")?;
                let current_read: Config = serde_json::from_str(&json_data)?;
                if current_read.muted == false && current_read.mute_list.contains(&new_message.author.id) == false{
                    if message_content.contains("lunko") {
                        new_message.react(ctx.http.clone(), ReactionType::from(EmojiId::from(1483594664190673028))).await?;
                    };
                    if message_content.contains("shinx") {
                        let shinx_list : [u64; 18] = [
                        1387115198560731338,
                        1387115199814828074,
                        1387115201261998160,
                        1387115203162013817,
                        1387115204692938954,
                        1387115524198105200,
                        1387115209038106725,
                        1387115525737545728,
                        1387115214738161735,
                        1387115528451260607,
                        1387115218366369922,
                        1387115522792882290,
                        1387115192210685984,
                        1387115193598873770,
                        1387115195457081425,
                        1387115197201776790,
                        1452160133756092609,
                        1295227647164420177,];
                        let shinx: usize = rand::thread_rng().gen_range(0..=18);
                        new_message.react(ctx.http.clone(), ReactionType::from(EmojiId::from(shinx_list[shinx]))).await?;
                    };
                    let spawn = rand::thread_rng().gen_range(0..=current_read.lunko_chance);
                    if spawn == 1 {
                        if rand::thread_rng().gen_range(0..=15) == 10 {
                            let file = File::open("shinylunko.png").await?;
                            let attachment = CreateAttachment::file(&file, "shinylunko.png").await?;
                            let content = CreateMessage::default()
                                .add_file(attachment);
                            let guild = new_message.channel_id.to_channel(ctx.http.clone()).await.unwrap().guild().unwrap();
                            guild.send_message(ctx.http.clone(), content).await?;
                        } else {
                            let file = File::open("lunkoembed.png").await?;
                            let attachment = CreateAttachment::file(&file, "lunkoembed.png").await?;
                            let content = CreateMessage::default()
                                .add_file(attachment);
                            let guild = new_message.channel_id.to_channel(ctx.http.clone()).await.unwrap().guild().unwrap();
                            guild.send_message(ctx.http.clone(), content).await?;
                        }
                    };
                    if message_content.contains("thank you shidbot") {
                        let guild = new_message.channel_id.to_channel(ctx.http.clone()).await.unwrap().guild().unwrap();
                        let content = CreateMessage::default()
                            .content("<:shinx_joy:1387115209038106725>");
                        guild.send_message(ctx.http.clone(), content).await?;                        
                    }
            }
                if message_content.contains("1239") {
                    let msg_length = message_content.len();
                    if &new_message.content[(msg_length - 4)..(msg_length)] == "1239" && new_message.author.bot == false && new_message.content.len() <= 32{
                        if new_message.guild_id != None {
                            let partial_guild = new_message.guild_id.unwrap().to_partial_guild(ctx.http.clone()).await?;
                            let member_edit = EditMember::new()
                                .nickname(new_message.content.clone());
                            let mut log = String::new();
                            let _ = write!(&mut log, "New name set by {}: {}", new_message.author.id, new_message.content);
                            let _ = log::log_to_file(log);
                            partial_guild.edit_member(ctx.http.clone(), 739931053560430802, member_edit).await?;
                            let mut and_let_there_be = String::new();
                            let _ = write!(&mut and_let_there_be, "and {} said let there be: {}", new_message.author.to_string(), new_message.content);
                            let content = CreateMessage::default()
                                .content(and_let_there_be);
                            let guild = new_message.channel_id.to_channel(ctx.http.clone()).await.unwrap().guild().unwrap();
                            guild.send_message(ctx.http.clone(), content).await?;   
                        }
                    };
                };
            }
        _ => {}
    }
    Ok(())
}