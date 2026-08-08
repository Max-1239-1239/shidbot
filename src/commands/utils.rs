use tokio::{fs::File, io::AsyncWriteExt};
use poise::serenity_prelude as serenity;
use serenity::CreateAttachment;
use poise::CreateReply;
use rand::{seq::SliceRandom};
use crate::{Context, Error, commands::log};

pub async fn random_image(
    pokemon: String, // valid: shinx, jolt
) -> Result<CreateReply , Error> {
    let mut search = {
        let files = std::fs::read_dir(format!("images/{}", pokemon))?;
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
            let file_path = format!("images/{}/{}", pokemon.clone(), &attachment.filename);
            let mut file = File::create(file_path.clone()).await?;
            let _ = file.write_all(&content).await; 
            let log_msg = format!("New image added to the {} folder by {}, path is: {}", pokemon.clone(), ctx.author(), file_path);
            let _ = log::log_to_file(log_msg);
        }
        msg.reply(ctx, "saved!").await?;
    };
    Ok(())
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