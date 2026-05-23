# A discord bot developed for Shidmark Percision Engineering.

Has a variety of commands ranging from shitposts to utilities.  
  
## Running your own Shidbot

I don't suggest doing it. You'll need to make some code changes, as some things are hard-coded for Shidmark. But if you really want to:

To start, build the source code using Cargo. Instructions for doing that can be found in Rust's documentation.  
  
Afterawrds, there are a few extra steps, as the bot depends on some external files to run. Just create them, format them if needed, and put them in the same directory as the binary.  

- `token.txt`: Just make a file called token.txt and paste in your bot token.

- Both images in the repository need to be in the same directory as the binary. Don't rename them without updating the code.

- You do not need to create a bot.log file, the bot will make one if it is not present.

- `config.json`: A JSON file with all the needed config elements. The bot will not make one if not present nor will it will not fix an improperly filled out one.  
  
This is the format for the JSON file. This will start the bot off with a spawn chance of 1/200, an empty mutelist, and nothing active.

```
{
  "lunko_chance": 200,
  "mute_list": [],
  "muted": false,
  "shidbot_alert_active": false,
  "custom_alert_active": false,
  "inactivity_alert_active": false
}
```


## TODO
- Figure out why `/customalert` stops after a varying period. 
