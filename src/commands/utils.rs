use async_std::task;
use ::serenity::model::{guild::PartialGuild, id::ChannelId};
use tokio::{fs::File, io::AsyncWriteExt};
use poise::serenity_prelude as serenity;
use serenity::CreateAttachment;
use poise::CreateReply;
use rand::{Rng, seq::SliceRandom};
use crate::{Context, Error, commands::{config_access::{self, ConfigOption}, log}};
use base64::{Engine, engine::general_purpose};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{fs, time::Duration};

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


#[derive(Deserialize, Debug, Serialize)]
pub struct StatusOptions {
    normal_lunko_status: Vec<String>,
    chris_nylon_lunko_status: Vec<String>,
    tech_connections_lunko_status: Vec<String>,
    dankpods_lunko_status: Vec<String>,
}

pub async fn startup(
    ctx: &serenity::Context,
) -> Result<(), Error> {
    config_access::config_edit(ConfigOption::StartupRun(true)).await?;
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
                let path = format!("dependancies/emojis/{}.png", emoji);
                let file = tokio::fs::read(path).await?;
                let base64_encoded_file = general_purpose::STANDARD.encode(file);
                let data_uri_string = format!("data:image/png;base64,{}", base64_encoded_file); 
                data_uri_string
            };
            ctx.create_application_emoji(emoji, &attachment).await?;
        };
    };
    log::log_to_file("Debug: Running startup commands".to_owned())?;
    loop { // this is the shidbot alert 
        let rand_duration = rand::thread_rng().gen_range(1..=604800); 
        log::log_to_file(format!("Debug: Time to next alert: {} seconds", rand_duration))?;
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

pub async fn random_image(
    pokemon: String, // valid: shinx, jolt
) -> Result<CreateReply , Error> {
    let mut search = {
        let files = std::fs::read_dir(format!("dependancies/{}", pokemon))?;
        let mut file_paths = Vec::new();
        for entry in files {
            file_paths.push(entry.unwrap().path());
        };
        file_paths
    };
    if search.len() > 0 {
        let shuffled_mons = {
        let mut rng = rand::thread_rng();
            search.shuffle(&mut rng);
            search.clone()
        };
        let attachment = {
            let mon = shuffled_mons[0].clone().into_os_string().into_string().unwrap();
            let file = File::open(&mon).await.expect("File not found.");
            let attachment = CreateAttachment::file(&file, &mon).await; 
            attachment    
        };
        let content = CreateReply::default()
            .attachment(attachment.expect("attachment not found"));
        Ok(content)
    } else {
        let content = CreateReply::default()
            .content(format!("no {} images found", pokemon.clone()));
        Ok(content)
    }
}

pub async fn add_to_collection(
    pokemon: String,
    ctx: Context<'_>, 
    msg: serenity::Message,
) -> Result<(), Error> {
    let attachments = msg.clone().attachments;
    if attachments.len() < 1 {
        msg.reply(ctx, "i can't find an image in this message").await?;
    } else {
        for attachment in &msg.attachments {
            let content = match attachment.download().await {
                Ok(content) => content,
                Err(why) => {
                    msg.reply(ctx, "something went wrong when downloading").await?;
                    let _ = log::log_to_file(why.to_string());
                    return Ok(());
                }
            };
            let file_path = format!("dependancies/{}/{}", pokemon.clone(), &attachment.filename);
            let mut file = File::create(file_path.clone()).await?;
            let _ = file.write_all(&content).await; 
            let log_msg = format!("New image added to the {} folder by {}, path is: {}", pokemon.clone(), ctx.author(), file_path);
            let _ = log::log_to_file(log_msg);
        }
        msg.reply(ctx, "saved!").await?;
    };
    Ok(())
}

pub fn get_status(
    status_type: usize, // desired response, index starts at 0, in order of order in status.json
) -> String {
    let json_data = fs::read_to_string("dependancies/data/status.json").unwrap();
    let current_read: StatusOptions = serde_json::from_str(&json_data).expect("failed to get json data");
    let mut target_list = match status_type {
        1 => {
            current_read.chris_nylon_lunko_status
        }, // chris nylon
        2 => {
            current_read.tech_connections_lunko_status
        }, // tech connections
        3 => {
            current_read.dankpods_lunko_status
        }, // dankpods
        _ => { // 0 or other, returns a normal lunko status
            current_read.normal_lunko_status
        }
    };
    let random_status = {
        let mut rng = rand::thread_rng();
        target_list.shuffle(&mut rng);
        target_list[0].clone()
    };
    return random_status;
}

pub fn timestamp_since(
    time: usize,
) -> String {
    let mut times = vec![];
    let mut push_value = time / 31557600;
    times.push(push_value);
    push_value = time / 2629800;
    times.push(push_value);
    while times[1] >= 12 {
        times[1] = times[1] - 12;
    };
    let timestamp = format!("{} years, {} months", times[0], times[1]);
    return timestamp;
}