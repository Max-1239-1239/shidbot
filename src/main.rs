#![warn(clippy::str_to_string)]

/*
Welcome to Shidbot's source code! Feel free to ask me (max1239) about anything in the code!

You'll notice there are four source code files in here, here's a quick description on what each one is:
    main.rs (you are here!) -> The main file. Handles bot startup & runtime + shidbot alert + keyword responses
    commands.rs -> The commands file, defines the functions for every command that Shidbot has
    keywords.rs -> The keywords file, defines the functions that run in response to a message keyword
    log.rs -> A really basic log module. Has one function which takes a message argument, prefixes it with a timestamp, and writes to the bot.log file
    config_access.rs -> A selection of functions related to handling Shidbot's config file
    utils.rs -> A selection of utility functions
*/

mod commands;
mod keywords;

use base64::engine::general_purpose;
use commands::log;
use commands::config_access;

use async_std::task;
use poise::{serenity_prelude as serenity};
use rand::Rng;
use ::serenity::model::id::UserId;
use ::serenity::{all::model::{guild::PartialGuild, id::ChannelId}};
use tokio::fs;
use std::{fmt::Write, io::Read, sync::Arc, time::Duration};
use std::{fs::File as TokenFile};
use base64::prelude::*;
use crate::commands::config_access::ConfigOption;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

const APPLICATION_EMOJI_NAME_LIST: [&'static str; 17] = [ // a list of application emojis, used to make them on startup if needed 
    ("shinx_shouting"),
    ("shinx_tearyeyed"),
    ("shinx_inspired"),
    ("shinx_determined"),
    ("shinx_dizzy"),
    ("shinx_shocked"),
    ("shinx_joy"),
    ("shinx_stunned"),
    ("shinx_angry"),
    ("shinx_sigh"),
    ("shinx_sad"),
    ("shinx_crying"),
    ("shinx_pain"),
    ("shinx_normal"),
    ("shinx_worried"),
    ("shinx_happy"),
    ("lunko")
];

#[derive(Debug)]
pub struct Data {
    // this is just kinda here as part of the quickstart stuff, its presence seems important but i dont use it
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error), // Catches errors during startup
        poise::FrameworkError::Command { error, ctx, .. } => { // Catches & logs errors in commands
            let mut error_msg = String::new();
            let _ = write!(&mut error_msg, "Error in command `{}`: {:?}", ctx.command().name, error,);
            let _ = log::log_to_file(error_msg);
        },
        poise::FrameworkError::CommandPanic { payload, ctx , .. } => { // Catches & logs panics in commands
            match payload{
                Some(t) => {
                    let _ = log::log_to_file(format!("Panic in command `{}`: {:?}", ctx.command().name, t));
                }
                None => {
                    let _ = log::log_to_file(format!("Panic in command `{}`", ctx.command().name));
                }
            }
        },
        error => { // Catches errors when handling errors
            if let Err(e) = poise::builtins::on_error(error).await {
                let err_msg = format!("Error while handling error: {}", e);
                let _ = log::log_to_file(err_msg);
            }
        },
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
            commands::whoisglobal(),
            commands::scramble(),
            commands::reverse(),
            commands::customalert(),
            commands::inactivityalert(),
            commands::shinx(),
            commands::shinx_collection(),
            commands::unshid(),
            commands::jolteon(),
            commands::jolteon_collection(),
        ], 
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
    let config_file_check = fs::read("config.json").await; 
    match config_file_check {
        Ok(_) => {} // File exists, pass
        Err(_) => { // File does not exist, make one w/ default values
            let _ = config_access::config_setup().await;
        }
    }
    let mut token = String::new();
    let _ = TokenFile::open("token.txt").unwrap().read_to_string(&mut token); // Grabs token from file & writes it to the token variable
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
        serenity::FullEvent::Ready { data_about_bot: _ } => { // Shidbot alert & emoji creation (if needed), begins on startup
            let application_emojis = ctx.http.get_application_emojis().await?;
            let mut emoji_list = Vec::new();
            let existing_emoji_list = {
                for emoji in application_emojis {
                    emoji_list.push(emoji.name);
                }
                emoji_list
            };
            for emoji in APPLICATION_EMOJI_NAME_LIST {
                if !existing_emoji_list.iter().any(|name| name == emoji) { // If emoji doesn't exist, shidbot will make it
                    let attachment = {
                        let path = format!("images/emojis/{}.png", emoji);
                        let file = fs::read(path).await?;
                        let base64_encoded_file = general_purpose::STANDARD.encode(file);
                        let data_uri_string = format!("data:image/png;base64,{}", base64_encoded_file); 
                        data_uri_string
                    };
                    ctx.create_application_emoji(emoji, &attachment).await?;
                };
            };
            loop { // this is the shidbot alert 
                let rand_duration = rand::thread_rng().gen_range(1..=604800); 
                task::sleep(Duration::from_secs(rand_duration)).await;
                let muted_status = {
                    match config_access::config_read(2).unwrap() {
                        ConfigOption::Muted(x) => {
                            let current = x;
                            current
                        },
                        _ => {
                            let current: bool = false;
                            current
                        },
                    }
                };
                if !muted_status {
                    let guild_channel = {
                        match config_access::config_read(7).unwrap() {
                            ConfigOption::ShidbotAlertTargetChannel(x, y, z) => {
                                let current = (x, y, z);
                                current
                            },
                            _ => {
                                let current: (u64, u64, u64) = (0, 0, 0);
                                current
                            },
                        }
                    };
                    let channel = {
                        let guild = PartialGuild::get(&ctx.http, guild_channel.0).await;
                        match guild {
                            Ok(_) => {}
                            Err(_) => {
                                return Ok(())
                            }
                        }
                        let guild = guild.unwrap();
                        let channelid = ChannelId::new(guild_channel.1);
                        let channels = guild.channels(&ctx.http).await;
                        match channels {
                            Ok(_) => {}
                            Err(_) => {
                                return Ok(())
                            }
                        }
                        let channels = channels.unwrap();
                        let channel = channels[&channelid].clone();
                        channel
                        };
                    let ping = format!("<@&{}>", guild_channel.2);
                    channel.say(&ctx.http, ping).await?;
                }        
            }
        }
        serenity::FullEvent::Message { new_message } => { // Message keyword responses
            let message_content = new_message.content.to_lowercase();
            let muted_status = {
                match config_access::config_read(2).unwrap() {
                    ConfigOption::Muted(x) => {
                        let current = x;
                        current
                    },
                    _ => {
                        let current: bool = false;
                        current
                    },
                }
            };
            let current_list = {
                match config_access::config_read(1).unwrap() {
                    ConfigOption::MuteList(x) => {
                        let current = x;
                        current
                    },
                    _ => {
                        let current: Vec<UserId> = Vec::new();
                        current
                    },
                }
            };
            if !muted_status && !current_list.contains(&new_message.author.id) {
                keywords::lunko_spawn(ctx.clone(), new_message.clone()).await?;
                if message_content.contains("lunk") {
                    keywords::lunk_keyword(ctx.clone(), new_message.clone()).await?;
                }
                if message_content.contains("shinx") {
                    keywords::shinx_keyword(ctx.clone(), new_message.clone(), message_content.clone()).await?;
                };
                if message_content.contains("thank you shidbot") { // thank you shidbot :)
                    keywords::thank_you_shidbot(ctx.clone(), new_message.clone()).await?;
                };
                if message_content.contains("thank") && message_content.contains("trac") {
                    keywords::thank_you_tracy(ctx.clone(), new_message.clone()).await?;
                }
                if message_content.contains("hello shidbot") {
                    keywords::hello_shidbot(ctx.clone(), new_message.clone()).await?;
                }
            }
                if message_content.contains("1239") {
                    keywords::one_two_three_nine(ctx.clone(), new_message.clone(), message_content.clone()).await?;
                };
                
            }
        _ => {} // Catch-all for other events
    }
    Ok(())
    
}