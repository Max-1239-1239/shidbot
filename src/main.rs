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

use commands::log;
use commands::config_access;

use poise::{serenity_prelude as serenity};
use ::serenity::model::id::UserId;
use tokio::fs;
use std::{fmt::Write, io::Read, sync::Arc, time::Duration};
use std::{fs::File as TokenFile};
use crate::commands::config_access::ConfigOption;
use crate::commands::utils;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

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
    let config_file_check = fs::read("dependancies/data/config.json").await; 
    match config_file_check {
        Ok(_) => {} // File exists, pass
        Err(_) => { // File does not exist, make one w/ default values
            let _ = config_access::config_setup().await;
        }
    }
    let _ = config_access::config_edit(ConfigOption::StartupRun(false)).await;
    let mut token = String::new();
    let _ = TokenFile::open("dependancies/data/token.txt").unwrap().read_to_string(&mut token); // Grabs token from file & writes it to the token variable
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::ClientBuilder::new(&token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    let run_startup = {
        match config_access::config_read(8).unwrap() {
            ConfigOption::StartupRun(x) => {
                let current = x;
                current
            },
            _ => {
                let current: bool = false;
                current
            },
        }
    };
    if !run_startup {
        utils::startup(ctx).await?;
    }
    match event {
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
                    if !&new_message.author.bot {
                        keywords::lunko_status_response(0, ctx.clone(), new_message.clone()).await?;
                    }
                }
                if message_content.contains("shinx") {
                    keywords::shinx_keyword(ctx.clone(), new_message.clone(), message_content.clone()).await?;
                };
                if message_content.contains("thank you shidbot") { // thank you shidbot :)
                    keywords::thank_you_shidbot(ctx.clone(), new_message.clone()).await?;
                };
                if message_content.contains("hello shidbot") {
                    keywords::hello_shidbot(ctx.clone(), new_message.clone()).await?;
                }
                if message_content.contains("chris nylon") {
                    keywords::lunko_status_response(1, ctx.clone(), new_message.clone()).await?;
                }
                if message_content.contains("technology connections") {
                    keywords::lunko_status_response(2, ctx.clone(), new_message.clone()).await?;
                }
                if message_content.contains("dankpods") || message_content.contains("frank") {
                    keywords::lunko_status_response(3, ctx.clone(), new_message.clone()).await?;
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