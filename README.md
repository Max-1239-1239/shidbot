# A discord bot developed for Shidmark Percision Engineering.

Has a variety of commands ranging from shitposts to utilities.  
  
# Running your own Shidbot

If you really want to run your own Shidbot instance, it's not too hard. Follow the instructions below and you should have the bot set up in no time!

## Step 1: Setting Up The Bot

To start, make an application in Discord's developer portal. Then, move to the "Bot" tab, and make a bot there. Give it all three intents and save your token somewhere safe. **Your bot token is the SOLE credential needed to access your bot! Be extremely careful with it!**  
Next, go to the Oauth2 tab and scroll to the URL generator. Pick "bot" and nothing else, and paste the generated link into your browser to add your bot to a server. You need Manage Server permissions to add a bot to a server.   
  
You're done with this step! You have a cool bot now!

## Step 2: Building From Source

I don't provide compiled binaries, so you'll need to compile it yourself. The easiest way is using Cargo, which you can install on Rust's website. Install it, navigate to the downloaded repository, and run `cargo build --release` to build a binary for your platform. This will take a few minutes and needs some decent computational power, so be patient. **Note: I cannot guarantee that Shidbot will work on all operating systems! It is only tested on Linux. I highly doubt it will run on Windows due to the way file paths are handled, and honestly I have no clue how Mac does anything.**

## Step 3: Setting Shidbot Up

Once you've built it, there should be a folder called "target" in Shidbot's directory. Put this wherever you want, then put the "images" folder from the repository in the same directory. Finally, make a new file in that directory called "token.txt". Paste your bot token in this file, make sure there's no whitespace at the beginning or end, then save it.  
  
This is all you need! Run the script and your bot should go online. If it doesn't, run the script in a terminal to catch errors.




