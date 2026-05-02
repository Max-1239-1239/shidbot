use crate::{Context, Error};
use serenity::CreateAttachment;
use poise::CreateReply;
use serenity::CreateEmbed;
use poise::serenity_prelude as serenity;
use rand::Rng;
use ::serenity::all::Colour;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::{fs};
use serde::{Deserialize, Serialize};
use serde_json;
use async_std::task::{self};
use std::time::{Duration, SystemTime};
use lumelog::{info};

#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    lunko_chance: u64,
    mute_list: Vec<serenity::UserId>,
    muted: bool,
    shidbot_alert_active: bool,
    custom_alert_active: bool,
    inactivity_alert_active: bool,
}

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
            extra_text_at_bottom: "I'm shidbot, a bot written in Rust running on a raspberry pi 3 model b+! \nContact max1239 for help/suggestions",
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
    let file = File::open("lunkoembed.png").await?;
    let attachment = CreateAttachment::file(&file, "lunkoembed.png").await?;
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
        let mut log = "message echoed by ".to_string();
        log.push_str(&ctx.author().id.to_string());
        log.push_str("; content is \"");
        log.push_str(&message);
        log.push_str("\"");
        info!(&log);
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
    let json_data = fs::read_to_string("config.json")?;
    let current_read: Config = serde_json::from_str(&json_data)?;
    let target_user = ctx.author().id;
    let new_list: Vec<serenity::UserId> = if current_read.mute_list.clone().into_iter().position(|x: serenity::UserId| x == target_user) != None{
        let position = (current_read.mute_list.clone().into_iter().position(|x: serenity::UserId| x == target_user)).expect("something bad happened ln 90");
        let mut new_list = current_read.mute_list;
        new_list.remove(position);
        ctx.say("you have been removed from the list").await?;
        new_list
    } else {
        let mut new_list: Vec<serenity::UserId> = current_read.mute_list;
        new_list.push(target_user);
        ctx.say("you have been added to the list").await?;
        new_list
    };
    let config_file = Config {
        lunko_chance: current_read.lunko_chance,
        mute_list: new_list,
        muted: current_read.muted,
        shidbot_alert_active: current_read.shidbot_alert_active,
        custom_alert_active: current_read.custom_alert_active,
        inactivity_alert_active: current_read.inactivity_alert_active,
    };
    let json_data = serde_json::to_string_pretty(&config_file).unwrap();
    let mut file = File::create("config.json").await?;
    file.write_all(json_data.as_bytes()).await?;
    Ok(())
}

/// Temporarily mute shidbot's random events [Mod Only]
#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn mute(
    ctx: Context<'_>,
    duration: u64,
) -> Result<(), Error> {
    let role_list=  ctx.author_member().await.unwrap().roles(&ctx.cache()).unwrap();
    let id: u64 = 869998894644351056;
    let moderator_role = ctx.guild_id().unwrap().role(ctx.http(), serenity::RoleId::from(id)).await?;
    if role_list.contains(&moderator_role) {
        ctx.say("mute started").await?;
        let json_data = fs::read_to_string("config.json")?;
        let current_read: Config = serde_json::from_str(&json_data)?;
        let config_file = Config {
            lunko_chance: current_read.lunko_chance,
            mute_list: current_read.mute_list,
            muted: true,
            shidbot_alert_active: current_read.shidbot_alert_active,
            custom_alert_active: current_read.custom_alert_active,
            inactivity_alert_active: current_read.inactivity_alert_active,
        };
        let json_data = serde_json::to_string_pretty(&config_file).unwrap();
        let mut file = File::create("config.json").await?;
        file.write_all(json_data.as_bytes()).await?;
        task::sleep(Duration::from_secs(duration)).await;
        let config_file = Config {
            lunko_chance: current_read.lunko_chance,
            mute_list: config_file.mute_list,
            muted: false,
            shidbot_alert_active: current_read.shidbot_alert_active,
            custom_alert_active: current_read.custom_alert_active,
            inactivity_alert_active: current_read.inactivity_alert_active,
        };
        let json_data = serde_json::to_string_pretty(&config_file).unwrap();
        let mut file = File::create("config.json").await?;
        file.write_all(json_data.as_bytes()).await?;
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
    let bot_admins: [u64; 2] = [739931053560430802, 296273378636201985];
    if bot_admins.contains(&u64::from(ctx.author().id)) {
        if new_spawn_chance != None {
            let json_data = fs::read_to_string("config.json")?;
            let current_read: Config = serde_json::from_str(&json_data)?;
            let config_file = Config {
                lunko_chance: new_spawn_chance.unwrap(),
                mute_list: current_read.mute_list,
                muted: current_read.muted,
                shidbot_alert_active: current_read.shidbot_alert_active,
                custom_alert_active: current_read.custom_alert_active,
                inactivity_alert_active: current_read.inactivity_alert_active,
            };
            let json_data = serde_json::to_string_pretty(&config_file).unwrap();
            let mut file = File::create("config.json").await?;
            file.write_all(json_data.as_bytes()).await?;
            ctx.say("spawn chance changed").await?;
        };
    } else {
        ctx.say("only bot admins can change this").await?;
    }
    Ok(())
}

/// Start the shidbot alert
#[poise::command(prefix_command, slash_command)]
pub async fn shidbotalert(
    ctx: Context<'_>,
) -> Result<(), Error> {
    loop {
        let rand_duration = rand::thread_rng().gen_range(1..=604800); 
        task::sleep(Duration::from_secs(rand_duration)).await;
        let json_data = fs::read_to_string("config.json")?;
        let current_read: Config = serde_json::from_str(&json_data)?;
        if current_read.muted == false {
            let channel = ctx.guild_channel().await.unwrap();
            channel.say(&ctx.http(), "<@&1453436782627520777>").await?;
        }        
    }
}

///Ban a user [Mod Only]
#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn ban(
    ctx: Context<'_>,
    target: serenity::User,
    reason: String,
) -> Result<(), Error> {
    let role_list=  ctx.author_member().await.unwrap().roles(&ctx.cache()).unwrap();
    let id: u64 = 869998894644351056;
    let moderator_role = ctx.guild_id().unwrap().role(ctx.http(), serenity::RoleId::from(id)).await?;
    if role_list.contains(&moderator_role) {
        ctx.partial_guild().await.unwrap().ban_with_reason(&ctx.http(), &target, 0, reason).await?;
        ctx.say("user banned").await?;
        let mut log = "user banned by ".to_string();
        log.push_str(&ctx.author().id.to_string());
        log.push_str("; content is \"");
        log.push_str(&target.id.to_string());
        log.push_str("\"");
        info!(&log);
    } else {
        ctx.say("moderator only command").await?;
    };
    Ok(())
}

/// Get information on someone outside of the server
#[poise::command(prefix_command, slash_command)]
pub async fn whothefuck(
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
    let account_age = serenity::Timestamp::from_unix_timestamp((SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap() - Duration::from_secs(user_id.created_at().unix_timestamp() as u64)).as_secs() as i64).unwrap();
    let mut account_age_string = (account_age.unix_timestamp() / 31557600).to_string();
    account_age_string.push_str(&format!("{}", account_age.format(" years, %-m months, %e days")).to_string());

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
    let mut account_age_string = (account_age.unix_timestamp() / 31557600).to_string();
    account_age_string.push_str(&format!("{}", account_age.format(" years, %-m months, %e days")).to_string());

    let server_age = serenity::Timestamp::from_unix_timestamp((SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap() - Duration::from_secs(user.joined_at.unwrap().unix_timestamp() as u64)).as_secs() as i64).unwrap();
    let mut server_age_string = (server_age.unix_timestamp() / 31557600).to_string();
    server_age_string.push_str(&format!("{}", server_age.format(" years, %-m months, %e days")).to_string());

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
    let json_data = fs::read_to_string("config.json")?;
    let current_read: Config = serde_json::from_str(&json_data)?;
    if cancel == None {
        let config_file = Config {
            lunko_chance: current_read.lunko_chance,
            mute_list: current_read.mute_list,
            muted: current_read.muted,
            shidbot_alert_active: current_read.shidbot_alert_active,
            custom_alert_active: true,
            inactivity_alert_active: current_read.inactivity_alert_active,
        };
        let json_data = serde_json::to_string_pretty(&config_file).unwrap();
        let mut file = File::create("config.json").await?;
        file.write_all(json_data.as_bytes()).await?;
        ctx.say("alert started").await?;
    } else if cancel.unwrap() == true{
        let config_file = Config {
            lunko_chance: current_read.lunko_chance,
            mute_list: current_read.mute_list,
            muted: current_read.muted,
            shidbot_alert_active: current_read.shidbot_alert_active,
            custom_alert_active: false,
            inactivity_alert_active: current_read.inactivity_alert_active,
        };
        let json_data = serde_json::to_string_pretty(&config_file).unwrap();
        let mut file = File::create("config.json").await?;
        file.write_all(json_data.as_bytes()).await?;
        ctx.say("alert cancelled").await?;
    } else {
        let config_file = Config {
            lunko_chance: current_read.lunko_chance,
            mute_list: current_read.mute_list,
            muted: current_read.muted,
            shidbot_alert_active: current_read.shidbot_alert_active,
            custom_alert_active: true,
            inactivity_alert_active: current_read.inactivity_alert_active,
        };
        let json_data = serde_json::to_string_pretty(&config_file).unwrap();
        let mut file = File::create("config.json").await?;
        file.write_all(json_data.as_bytes()).await?;
        ctx.say("alert started").await?;
    };
    loop {
        let rand_duration = rand::thread_rng().gen_range(1..=frequency);
        task::sleep(Duration::from_secs(rand_duration)).await;
        let json_data = fs::read_to_string("config.json")?;
        let current_read: Config = serde_json::from_str(&json_data)?;
        if current_read.custom_alert_active == false {
            break;
        }
        if current_read.muted == false {
            let mut ping_string = "<@".to_string();
            let user_id_string = user.id.get().to_string();
            ping_string.push_str(&user_id_string);
            ping_string.push_str(">");
            ctx.say(ping_string).await?;
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

#[poise::command(slash_command, prefix_command, guild_only)] 
pub async fn inactivityalert(
    ctx: Context<'_>,
    message: String,
    cancel: Option<bool>
) -> Result<(), Error> {
    let ref_guild = ctx.guild_channel().await.unwrap();
    let mut elapsed_min: u32 = 0;
    if cancel != None {
        let json_data = fs::read_to_string("config.json")?;
        let current_read: Config = serde_json::from_str(&json_data)?;
        let config_file = Config {
            lunko_chance: current_read.lunko_chance,
            mute_list: current_read.mute_list,
            muted: current_read.muted,
            shidbot_alert_active: current_read.shidbot_alert_active,
            custom_alert_active: current_read.custom_alert_active,
            inactivity_alert_active: false,
        };
        let json_data = serde_json::to_string_pretty(&config_file).unwrap();
        let mut file = File::create("config.json").await?;
        file.write_all(json_data.as_bytes()).await?;
    } else {
        let json_data = fs::read_to_string("config.json")?;
        let current_read: Config = serde_json::from_str(&json_data)?;
        let config_file = Config {
            lunko_chance: current_read.lunko_chance,
            mute_list: current_read.mute_list,
            muted: current_read.muted,
            shidbot_alert_active: current_read.shidbot_alert_active,
            custom_alert_active: current_read.custom_alert_active,
            inactivity_alert_active: true,
        };
        let json_data = serde_json::to_string_pretty(&config_file).unwrap();
        let mut file = File::create("config.json").await?;
        file.write_all(json_data.as_bytes()).await?;
    };
    let last_id = ref_guild.last_message_id.unwrap(); 
    loop {
        task::sleep(Duration::from_mins(10)).await; 
        let json_data = fs::read_to_string("config.json")?;
        let current_read: Config = serde_json::from_str(&json_data)?;
        if current_read.inactivity_alert_active == false {
            break;
        }
        let ref_guild = ctx.guild_channel().await.unwrap();
        if ref_guild.last_message_id.unwrap() == last_id { 
            elapsed_min = elapsed_min + 10;
        } else {
            ctx.rerun().await?;
            break;
        };
        if elapsed_min >= 600 { 
            ref_guild.say(&ctx.http(), &message).await?;
            elapsed_min = 0;
            continue;
        } else {
            continue;
        };
    }
    Ok(())
}