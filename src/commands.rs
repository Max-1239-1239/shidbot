#![allow(deprecated)]

use crate::{Context, Error, commands::config_access::ConfigOption};
use serenity::CreateAttachment;
use poise::CreateReply;
use serenity::CreateEmbed;
use poise::serenity_prelude as serenity;
use rand::{Rng, seq::SliceRandom};
use ::serenity::{all::Colour, builder::GetMessages, model::{guild::PartialGuild, id::{ChannelId, UserId}}};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::{fmt::Write};
use async_std::task::{self};
use std::time::{Duration, SystemTime};
pub mod log;
pub mod config_access;

/// Get information about shidbot
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn about(
    ctx: Context<'_>,
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "I'm shidbot, a discord bot written in Rust! \nDeveloped by max1239, contact him for suggestions or bug reports! \nI'm also open source, see my github repository at github.com/Max-1239-1239/shidbot",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// Ping shidbot
#[poise::command(prefix_command, slash_command)]
pub async fn ping(
    ctx: Context<'_>,
) -> Result<(), Error> {
    if rand::thread_rng().gen_range(1..=50) == 25 {
        ctx.say("shidbot is offline").await?;
    } else {
        ctx.say("shidbot is online").await?;
    }
    Ok(())
}

/// Lunko
#[poise::command(prefix_command, slash_command)]
pub async fn lunko(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let file = File::open("images/lunkoembed.png").await?;
    let attachment = CreateAttachment::file(&file, "images/lunkoembed.png").await?;
    let content = CreateReply::default()
        .attachment(attachment);
    ctx.send(content).await?;
    Ok(())
}

/// Send a message using shidbot
#[poise::command(slash_command, guild_only)]
pub async fn echo(
    ctx: Context<'_>,
    channel: serenity::GuildChannel,
    message: String,
) -> Result<(), Error> {
    let member = ctx.author_member().await.unwrap();
    let permissions = ctx.partial_guild().await.unwrap().user_permissions_in(&channel, &member);
    if permissions.send_messages() == true {
        channel.say(&ctx.http(), &message).await?;
        let mut log = String::new();
        let _ = write!(&mut log, "Message echoed by {}, content is: {}.", ctx.author().id, message);
        let _ = log::log_to_file(log);
        ctx.say("done").await?;
    } else {
        ctx.say("do it yourself").await?;
    }
    Ok(())
}

/// Add/Remove yourself from shidbot's mute list
#[poise::command(prefix_command, slash_command)]
pub async fn mutelist(
    ctx: Context<'_>,
) -> Result<(), Error> {
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
    let target_user = ctx.author().id;
    let new_list: Vec<serenity::UserId> = if current_list.clone().into_iter().position(|x: serenity::UserId| x == target_user) != None{
        let position = (current_list.clone().into_iter().position(|x: serenity::UserId| x == target_user)).expect("something bad happened ln 90");
        let mut new_list = current_list;
        new_list.remove(position);
        ctx.say("you have been removed from the list").await?;
        new_list
    } else {
        let mut new_list: Vec<serenity::UserId> = current_list;
        new_list.push(target_user);
        ctx.say("you have been added to the list").await?;
        new_list
    };
    config_access::config_edit(ConfigOption::MuteList(new_list)).await?;
    Ok(())
}

/// Temporarily mute shidbot's random events [Requires mute permissions]
#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn mute(
    ctx: Context<'_>,
    duration: u64,
) -> Result<(), Error> {
    let member_permissions = ctx.author_member().await.unwrap().permissions(&ctx.cache()).unwrap();
    if member_permissions.mute_members() {
        ctx.say("mute started").await?;
        config_access::config_edit(ConfigOption::Muted(true)).await?;
        task::sleep(Duration::from_secs(duration)).await;
        config_access::config_edit(ConfigOption::Muted(false)).await?;
        ctx.say("mute ended").await?;
    } else {
        ctx.say("you need to be a moderator to use this").await?;
    }
    Ok(())
}

/// Configure shidbot [Bot Admin Only]
#[poise::command(prefix_command, slash_command)]
pub async fn config(
    ctx: Context<'_>,
    new_spawn_chance: Option<u64>,
) -> Result<(), Error> {
    let bot_admins = {
        match config_access::config_read(5).unwrap() {
            ConfigOption::BotAdmins(x) => {
                let current = x;
                current
            },
            _ => {
                let current: Vec<UserId> = Vec::new();
                current
            },
        }
    };
    if bot_admins.contains(&ctx.author().id) {
        if new_spawn_chance != None {
            config_access::config_edit(ConfigOption::LunkoChance(new_spawn_chance.unwrap())).await?;
            ctx.say("spawn chance changed").await?;
        };
    } else {
        ctx.say("only bot admins can change this").await?;
    }
    Ok(())
}

///Ban a user [Requires ban permissions]
#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn ban(
    ctx: Context<'_>,
    target: serenity::User,
    reason: String,
) -> Result<(), Error> {
    let member_permissions = ctx.author_member().await.unwrap().permissions.unwrap();
    if member_permissions.ban_members() {
        let member = target.clone().member;
        let target_is_in_server = match member.clone() {
            Some(_x) => {
                true
            }
            None => {
                false
            }
        };
        if target_is_in_server {
            if member_permissions.ban_members() {
                ctx.say("this command cannot be used on others with ban permissions").await?;
                let log_msg = format!("Attempted use of `/ban` by: {} on a user with ban permissions | Target: {}", ctx.author().id, target.id.to_string());
                let _ = log::log_to_file(log_msg);
                return Ok(());
            }
        }
        ctx.partial_guild().await.unwrap().ban_with_reason(&ctx.http(), &target, 0, reason).await?;
        ctx.say("user banned").await?;
        let mut log = String::new();
        let _ = write!(&mut log, "User {} banned by {}.", target.id.to_string(), ctx.author().id,);
        let _ = log::log_to_file(log);
        return Ok(())
    } else {
        ctx.say("moderator only command").await?;
        return Ok(())
    };
}

/// Get information on someone outside of the server
#[poise::command(prefix_command, slash_command)]
pub async fn whoisglobal(
    ctx: Context<'_>,
    user_id: serenity::User,
) -> Result<(), Error> {
    let link = if user_id.avatar_url() != None {
        let link = user_id.avatar_url().unwrap();
        link
    } else {
        let link = user_id.default_avatar_url();
        link
    };
    let account_age = serenity::Timestamp::from_unix_timestamp((SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap() - Duration::from_secs(user_id.created_at().unix_timestamp() as u64)).as_secs() as i64).unwrap();    let years = account_age.unix_timestamp() / 31557600;
    let months = (account_age.unix_timestamp() / 2629746 ) - (years * 12);
    let days = (account_age.unix_timestamp() / 86400) - (years * 365) - (months * 30);
    let account_age_string = format!("{} years, {} months, {} days", years, months, days);

    let embed = CreateEmbed::new()
        .thumbnail(link)
        .title(&user_id.name)
        .field("ID", (&user_id.id).to_string(), true)
        .field("Account Created",format!("{}", &user_id.created_at().format("%B %e, %Y at %l:%M %p")).to_string(), true)
        .field("Account Age",account_age_string, true)
        .color(Colour::BLUE);
    let reply = CreateReply::default()
        .embed(embed);
    ctx.send(reply).await?;
    Ok(())
}

/// Get information on someone inside of the server
#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn whois(
    ctx: Context<'_>,
    user: serenity::Member,
) -> Result<(), Error> {
    let link = if user.user.avatar_url() != None {
        let link = user.user.avatar_url().unwrap();
        link
    } else {
        let link = user.user.default_avatar_url();
        link
    };
    let account_age = serenity::Timestamp::from_unix_timestamp((SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap() - Duration::from_secs(user.user.created_at().unix_timestamp() as u64)).as_secs() as i64).unwrap();
    let years = account_age.unix_timestamp() / 31557600;
    let months = (account_age.unix_timestamp() / 2629746 ) - (years * 12);
    let days = (account_age.unix_timestamp() / 86400) - (years * 365) - (months * 30);
    let account_age_string = format!("{} years, {} months, {} days", years, months, days);

    let server_age: serenity::Timestamp = serenity::Timestamp::from_unix_timestamp((SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap() - Duration::from_secs(user.joined_at.unwrap().unix_timestamp() as u64)).as_secs() as i64).unwrap();
    let years = server_age.unix_timestamp() / 31557600;
    let months = (server_age.unix_timestamp() / 2629746 ) - (years * 12);
    let days = (server_age.unix_timestamp() / 86400) - (years * 365) - (months * 30);
    let server_age_string = format!("{} years, {} months, {} days", years, months, days);

    let role_list=  user.roles(&ctx.cache()).unwrap();
    let mut length = role_list.len();
    let mut str_role_list_init = String::new();
    let str_role_list = loop {
        str_role_list_init.push_str(&role_list[length - 1].to_string());
        length = length - 1;
        if length == 0 {
            break str_role_list_init;
        } else {
            continue;
        };
    };

    let embed = CreateEmbed::new()
        .thumbnail(link)
        .title(&user.user.name)
        .field("ID", (&user.user.id).to_string(), true)
        .field("Account Created",format!("{}", &user.user.created_at().format("%B %e, %Y at %l:%M %p")).to_string(), true)
        .field("Account Age",account_age_string, true)
        .field("Joined Server",format!("{}", &user.joined_at.unwrap().format("%B %e, %Y at %l:%M %p")).to_string(), true)
        .field("Been In Server", server_age_string, true)
        .field("Roles", str_role_list, false)
        .color(Colour::BLUE);
    let reply = CreateReply::default()
        .embed(embed);
    ctx.send(reply).await?;
    Ok(())
}

/// Start a shidbot alert for a specific user
#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn customalert(
    ctx: Context<'_>,
    user: serenity::User,
    frequency: u64,
    cancel: Option<bool>,
) -> Result<(), Error> {
    if cancel == None {
        config_access::config_edit(ConfigOption::CustomAlertActive(true)).await?;
        ctx.say("alert started").await?;
    } else if cancel.unwrap() == true{
        config_access::config_edit(ConfigOption::CustomAlertActive(false)).await?;
        ctx.say("alert cancelled").await?;
    } else {
        config_access::config_edit(ConfigOption::CustomAlertActive(true)).await?;
        ctx.say("alert started").await?;
    };
    for _i in 1..=100 {
        let rand_duration = rand::thread_rng().gen_range(1..=frequency);
        task::sleep(Duration::from_secs(rand_duration)).await;
        let active_status = {
            match config_access::config_read(3).unwrap() {
                ConfigOption::CustomAlertActive(x) => {
                    let current = x;
                    current
                },
                _ => {
                    let current: bool = false;
                    current
                },
            }
        };
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
        if active_status == false {
            break;
        }
        if muted_status == false {
            let channel = {
                let guild_channel = {
                    match config_access::config_read(6).unwrap() {
                        ConfigOption::CustomAlertTargetChannel(x, y) => {
                            let current = (x, y);
                            current
                        },
                        _ => {
                            let current: (u64, u64) = (0, 0);
                            current
                        },
                    }
                };
                let guild = PartialGuild::get(&ctx.http(), guild_channel.0).await;
                match guild {
                    Ok(_) => {}
                    Err(_) => {
                        ctx.say("i couldn't find the configured guild, the bot may be improperly configured").await?;
                        return Ok(())
                    }
                }
                let guild = guild.unwrap();
                let channelid = ChannelId::new(guild_channel.1);
                let channels = guild.channels(&ctx.http()).await;
                match channels {
                    Ok(_) => {}
                    Err(_) => {
                        ctx.say("i couldn't find the configured channel, the bot may be improperly configured").await?;
                        return Ok(())
                    }
                }
                let channels = channels.unwrap();
                let channel = channels[&channelid].clone();
                channel
            };
            let ping_string = format!("<@{}>", user.id.get().to_string());
            channel.say(&ctx.http(), ping_string).await?;
        }        
    }    
    Ok(())
}

/// Translate text into pinglr-speak
#[poise::command(slash_command)]
pub async fn reverse(
    ctx: Context<'_>,
    phrase: String,
) -> Result<(), Error> {
    let reversed_word: String = phrase.chars().rev().collect();
    ctx.say(reversed_word).await?;
    Ok(())
}

/// Scramble text into completely useless gibberish
#[poise::command(slash_command)]
pub async fn scramble(
    ctx: Context<'_>,
    text: String,
) -> Result<(), Error> {
    let mut text_as_bytes = text.clone().into_bytes();
    let mut length = text_as_bytes.clone().len();
    let mut final_string: Vec<u8> = Vec::new();
    let final_string = loop {
        let rand_select = rand::thread_rng().gen_range(1..=length) - 1;
        final_string.push(text_as_bytes[rand_select]);
        text_as_bytes.remove(usize::from(rand_select) );
        length = length - 1;
        if length == 0 {
            break final_string;
        } else {
            continue;
        };
};
    let final_string = String::from_utf8(final_string).unwrap();
    ctx.say(final_string).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn inactivityalert( // this command is questionably functional, likely similar issues to /customalert
    ctx: Context<'_>,
    message: String,
    duration_minutes: u32,
    cancel: Option<bool>
) -> Result<(), Error> {
    let ref_guild = ctx.guild_channel().await.unwrap();
    let mut elapsed_min: u32 = 0;
    if cancel == None {
        config_access::config_edit(ConfigOption::InactivityAlertActive(true)).await?;
    } else if cancel.unwrap() {
        config_access::config_edit(ConfigOption::InactivityAlertActive(false)).await?;
    } else {
        config_access::config_edit(ConfigOption::InactivityAlertActive(true)).await?;
    };
    let last_id = ref_guild.last_message_id.unwrap(); 
    loop {
        task::sleep(Duration::from_mins(2)).await; 
        let active_status = {
            match config_access::config_read(4).unwrap() {
                ConfigOption::InactivityAlertActive(x) => {
                    let current = x;
                    current
                },
                _ => {
                    let current: bool = false;
                    current
                },
            }
        };
        if active_status == false {
            break;
        }
        let ref_guild = ctx.guild_channel().await.unwrap();
        if ref_guild.last_message_id.unwrap() == last_id { 
            elapsed_min = elapsed_min + 2;
        } else {
            ctx.rerun().await?;
            break;
        };
        if elapsed_min >= duration_minutes { 
            ref_guild.say(&ctx.http(), &message).await?;
            elapsed_min = 0;
            continue;
        } else {
            continue;
        };
    }
    Ok(())
}


/// Shinx
#[poise::command(slash_command, prefix_command)] 
pub async fn shinx(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let mut search = {
        let files = std::fs::read_dir("images/shinx")?;
        let mut file_paths = Vec::new();
        for entry in files {
            file_paths.push(entry.unwrap().path());
        };
        file_paths
    };
    if search.len() > 0 {
        let shuffled_shinxes = {
        let mut rng = rand::thread_rng();
            search.shuffle(&mut rng);
            search.clone()
        };
        let attachment = {
            let shinx = shuffled_shinxes[0].clone().into_os_string().into_string().unwrap();
            let file = File::open(&shinx).await?;
            let attachment = CreateAttachment::file(&file, &shinx).await?; 
            attachment    
        };
        let content = CreateReply::default()
            .attachment(attachment);
        ctx.send(content).await?;
    } else {
        ctx.say("couldn't find any shinx images to send").await?;
    };
    Ok(())
}

/// Add a new shinx image to the collection
#[poise::command(context_menu_command = "Add to shinx collection")]
pub async fn shinx_collection(
    ctx: Context<'_>, 
    msg: serenity::Message
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
            let file_path = format!("images/shinx/{}", &attachment.filename);
            let mut file = File::create(file_path.clone()).await?;
            let _ = file.write_all(&content).await; 
            let log_msg = format!("new image added to the folder by {}, path is: {}", ctx.author(), file_path);
            let _ = log::log_to_file(log_msg);
        }
        msg.reply(ctx, "saved!").await?;
    };
    Ok(())
}

/// Purge shidbot messages
#[poise::command(slash_command, prefix_command, guild_only)] 
pub async fn unshid(
    ctx: Context<'_>,
    messages_to_search: u8,
) -> Result<(), Error> {
    if messages_to_search > 100 || messages_to_search <= 1{
        ctx.say("number must be between 2 and 100").await?;
        return Ok(())
    }
    let channel_id = ctx.channel_id();
    let target_messages = {
        let builder = GetMessages::new().limit(messages_to_search);
        let messages = channel_id.messages(&ctx.http(), builder).await?;
        let mut targets = Vec::new();
        for message in messages {
            if message.author.id == 1389315953401266216 {
                targets.push(message.id)
            }
        };
        targets
    };
    let msg= format!("found {} messages, deleting", target_messages.len());
    ctx.say(msg).await?;
    channel_id.delete_messages(&ctx.http(), target_messages).await?;
    Ok(())
}