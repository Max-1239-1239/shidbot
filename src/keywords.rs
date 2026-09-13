use poise::{serenity_prelude as serenity};
use ::serenity::all::{CreateAttachment, CreateMessage, EditMember, ReactionType};
use rand::Rng;
use tokio::fs::File;

use crate::{Error, commands::{config_access::{self, ConfigOption}, log, utils}};

pub async fn shinx_keyword(
    ctx: serenity::Context, 
    new_message: serenity::Message,
    message_content: String,
) -> Result<(), Error> {
    let application_emojis = ctx.http.get_application_emojis().await?;
    match &message_content {
        msg if msg.contains("shinx_shouting") => {new_message.react(ctx.http.clone(), ReactionType::from(application_emojis[0].id)).await?;}
        msg if msg.contains("shinx_tearyeyed") => {new_message.react(ctx.http.clone(), ReactionType::from(application_emojis[1].id)).await?;}
        msg if msg.contains("shinx_inspired") => {new_message.react(ctx.http.clone(), ReactionType::from(application_emojis[2].id)).await?;}
        msg if msg.contains("shinx_determined") => {new_message.react(ctx.http.clone(), ReactionType::from(application_emojis[3].id)).await?;}
        msg if msg.contains("shinx_dizzy") => {new_message.react(ctx.http.clone(), ReactionType::from(application_emojis[4].id)).await?;}
        msg if msg.contains("shinx_shocked") => {new_message.react(ctx.http.clone(), ReactionType::from(application_emojis[5].id)).await?;}
        msg if msg.contains("shinx_joy") => {new_message.react(ctx.http.clone(), ReactionType::from(application_emojis[6].id)).await?;}
        msg if msg.contains("shinx_stunned") => {new_message.react(ctx.http.clone(), ReactionType::from(application_emojis[7].id)).await?;}
        msg if msg.contains("shinx_angry") => {new_message.react(ctx.http.clone(), ReactionType::from(application_emojis[8].id)).await?;}
        msg if msg.contains("shinx_sigh") => {new_message.react(ctx.http.clone(), ReactionType::from(application_emojis[9].id)).await?;}
        msg if msg.contains("shinx_sad") => {new_message.react(ctx.http.clone(), ReactionType::from(application_emojis[10].id)).await?;}
        msg if msg.contains("shinx_crying") => {new_message.react(ctx.http.clone(), ReactionType::from(application_emojis[11].id)).await?;}
        msg if msg.contains("shinx_pain") => {new_message.react(ctx.http.clone(), ReactionType::from(application_emojis[12].id)).await?;}
        msg if msg.contains("shinx_normal") => {new_message.react(ctx.http.clone(), ReactionType::from(application_emojis[13].id)).await?;}
        msg if msg.contains("shinx_worried") => {new_message.react(ctx.http.clone(), ReactionType::from(application_emojis[14].id)).await?;}
        msg if msg.contains("shinx_happy") => {new_message.react(ctx.http.clone(), ReactionType::from(application_emojis[15].id)).await?;}
        _ => {
            let shinx: usize = rand::thread_rng().gen_range(0..=15);
            new_message.react(ctx.http.clone(), ReactionType::from(application_emojis[shinx].id)).await?;
        }
    }
    Ok(())
}

pub async fn lunk_keyword(
    ctx: serenity::Context, 
    new_message: serenity::Message
) -> Result<(), Error> {
    let application_emojis = ctx.http.get_application_emojis().await?;
    new_message.react(ctx.http.clone(), ReactionType::from(application_emojis[16].clone())).await?;
    Ok(())
}

pub async fn lunko_spawn(
    ctx: serenity::Context,
    new_message: serenity::Message
) -> Result<(), Error> {
    let lunko_chance = {
        match config_access::config_read(0).unwrap() {
            ConfigOption::LunkoChance(x) => {
                let current = x;
                current
            },
            _ => {
                let current: u64 = 0;
                current
            },
        }
    };
    if rand::thread_rng().gen_range(0..=lunko_chance) == 1 {
        if rand::thread_rng().gen_range(0..=15) == 10 {
            let file = File::open("dependancies/shinylunko.png").await?;
            let attachment = CreateAttachment::file(&file, "shinylunko.png").await?;
            let content = CreateMessage::default()
                .add_file(attachment);
            let guild = new_message.channel_id.to_channel(ctx.http.clone()).await.unwrap().guild().unwrap();
            guild.send_message(ctx.http.clone(), content).await?;
        } else {
            let file = File::open("dependancies/lunkoembed.png").await?;
            let attachment = CreateAttachment::file(&file, "lunkoembed.png").await?;
            let content = CreateMessage::default()
                .content(utils::get_status(0))
                .add_file(attachment);
            let guild = new_message.channel_id.to_channel(ctx.http.clone()).await.unwrap().guild().unwrap();
            guild.send_message(ctx.http.clone(), content).await?;
        }
    };
    Ok(())
}

pub async fn thank_you_shidbot(
    ctx: serenity::Context,
    new_message: serenity::Message
) -> Result<(), Error> {
    let guild = new_message.channel_id.to_channel(ctx.http.clone()).await.unwrap().guild().unwrap();
    let application_emojis = ctx.http.get_application_emojis().await?;
    let emoji = format!("<:shinx_joy:{}>", application_emojis[6].id);
    let content = CreateMessage::default()
        .content(emoji);
    guild.send_message(ctx.http.clone(), content).await?;      
    Ok(())
}

pub async fn one_two_three_nine(
    ctx: serenity::Context,
    new_message: serenity::Message,
    message_content: String,
) -> Result<(), Error> {
    let msg_length = message_content.len();
    if &new_message.content[(msg_length - 4)..(msg_length)] == "1239" && new_message.author.bot == false && new_message.content.len() <= 32{
        if new_message.guild_id != None {
            let partial_guild = new_message.guild_id.unwrap().to_partial_guild(ctx.http.clone()).await?;
            let member_edit = EditMember::new()
                .nickname(new_message.content.clone());
            let log = format!("New name set by {}: {}", new_message.author.id, new_message.content);
            let _ = log::log_to_file(log);
            partial_guild.edit_member(ctx.http.clone(), 739931053560430802, member_edit).await?;
            let and_let_there_be = format!("and {} said: let there be {}", new_message.author, new_message.content);
            let content = CreateMessage::default()
                .content(and_let_there_be);
            let guild = new_message.channel_id.to_channel(ctx.http.clone()).await.unwrap().guild().unwrap();
            guild.send_message(ctx.http.clone(), content).await?;   
        }
    };
    Ok(())
}

pub async fn hello_shidbot(
    ctx: serenity::Context,
    new_message: serenity::Message,
) -> Result<(), Error> {
    let guild_channel = new_message.channel_id.to_channel(ctx.http.clone()).await.unwrap().guild().unwrap();
    let content = CreateMessage::default()
        .content(format!("hello {}", new_message.author.name));
    guild_channel.send_message(ctx.http.clone(), content).await?;      
    Ok(())
}