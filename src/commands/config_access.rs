use poise::serenity_prelude as serenity;
use crate::Error;
use serde::{Deserialize, Serialize};
use serde_json;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::fs;

#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    lunko_chance: u64, // 0
    mute_list: Vec<serenity::UserId>, // 1
    muted: bool, // 2
    custom_alert_active: bool, // 3
    inactivity_alert_active: bool, // 4
    bot_admins: Vec<serenity::UserId>, // 5
    custom_alert_target_channel: (u64, u64), // 6 | Format: (GuildId, ChannelId)
    shidbot_alert_target_channel: (u64, u64, u64), // 7 | Change to an invalid ID to disable shidbot alert (GuildId, Channelid, RoleId)
}
pub enum ConfigOption {
    LunkoChance(u64),
    MuteList(Vec<serenity::UserId>),
    Muted(bool),
    CustomAlertActive(bool),
    InactivityAlertActive(bool),
    BotAdmins(Vec<serenity::UserId>),
    CustomAlertTargetChannel(u64, u64),
    ShidbotAlertTargetChannel(u64, u64, u64),
}

pub fn config_read(field: u8) -> Result<ConfigOption , Error> { // Used to read a value from config files, return value needs to be unwrapped using a match statement
    let json_data = fs::read_to_string("config.json")?;
    let current_read: Config = serde_json::from_str(&json_data)?;
    match field {
        0 => {
            return Ok(ConfigOption::LunkoChance(current_read.lunko_chance));
        },
        1 => {
            return Ok(ConfigOption::MuteList(current_read.mute_list));
        },
        2 => {
            return Ok(ConfigOption::Muted(current_read.muted));
        },
        3 => {
            return Ok(ConfigOption::CustomAlertActive(current_read.custom_alert_active));
        },
        4 => {
            return Ok(ConfigOption::InactivityAlertActive(current_read.inactivity_alert_active));
        },
        5 => {
            return Ok(ConfigOption::BotAdmins(current_read.bot_admins));
        },
        6 => {
            return Ok(ConfigOption::CustomAlertTargetChannel(current_read.custom_alert_target_channel.0, current_read.custom_alert_target_channel.1));
        },
        7 => {
            return Ok(ConfigOption::ShidbotAlertTargetChannel(current_read.shidbot_alert_target_channel.0 , current_read.shidbot_alert_target_channel.1, current_read.shidbot_alert_target_channel.2));
        },
        _ => {
            panic!("Field argument out of bounds.");
        }
    }
}

pub async fn config_edit(new_value: ConfigOption) -> Result<(), Error> { // Used to edit config files
    let json_data = fs::read_to_string("config.json")?;
    let mut current_read: Config = serde_json::from_str(&json_data)?;
    match new_value {
        ConfigOption::LunkoChance(chance) => {current_read.lunko_chance = chance},
        ConfigOption::MuteList(user_ids) => {current_read.mute_list = user_ids},
        ConfigOption::Muted(active) => {current_read.muted = active},
        ConfigOption::CustomAlertActive(active) => {current_read.custom_alert_active = active},
        ConfigOption::InactivityAlertActive(active) => {current_read.inactivity_alert_active = active},
        ConfigOption::BotAdmins(user_ids) => {current_read.bot_admins = user_ids},
        ConfigOption::CustomAlertTargetChannel(guild_id, channel_id) => {current_read.custom_alert_target_channel = (guild_id, channel_id)},
        ConfigOption::ShidbotAlertTargetChannel(guild_id, channel_id, role_id) => {current_read.shidbot_alert_target_channel = (guild_id, channel_id, role_id)},
    }
    let json_data = serde_json::to_string_pretty(&current_read).unwrap();
    let mut file = File::create("config.json").await?;
    file.write_all(json_data.as_bytes()).await?;
    Ok(())
}

pub async fn config_setup() -> Result<(), Error> { // Runs at startup if there's no existing config file, makes one with default values
    let mut file = File::create("config.json").await?;
    let config_file = Config {
        lunko_chance: 100,
        mute_list: Vec::new(),
        muted: false,
        custom_alert_active: false,
        inactivity_alert_active: false,
        bot_admins: Vec::new(),
        custom_alert_target_channel: (0, 0), 
        shidbot_alert_target_channel: (0, 0, 0), 
    };
    let json_data = serde_json::to_string_pretty(&config_file).unwrap();
    file.write_all(json_data.as_bytes()).await?;
    Ok(())
}