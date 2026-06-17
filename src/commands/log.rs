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