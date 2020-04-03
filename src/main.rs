#![allow(unused_imports, unused_variables, unused_mut, unused_attributes, unused_parens,)]

mod util;
mod listeners;
mod commands;

// command imports
use commands::fun::roll::*;
use commands::util::help::*;

use serenity::{
    client::{
        Client,
        bridge::gateway::ShardManager,
    },
    framework::{
        standard::{
            macros::group,
            DispatchError,
        },
        StandardFramework,
    },
    prelude::{
        Mutex,
        TypeMapKey,
    }
};

use log::error;

#[macro_use]
extern crate dotenv_codegen;

use std::{
    env,
    collections::HashSet,
    sync::Arc,
};

use util::{
    db::{
        new_database,
        get_prefix,
    }
};

use listeners::handler::Handler;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

// command structs
#[group]
#[description = "Fun commands."]
#[commands(roll)]
struct Fun;

pub fn main() {
    let token = dotenv!("TOKEN");

    let mut client = Client::new(&token, Handler).expect("Coult not create Discord Client!");

    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    // let mut owners = HashSet::new();
    // owners.insert(dotenv!("CANARADO").to_string());
    // let bot_id = dotenv!("CURRENT_BOT_ID").to_string();

    let (owners, bot_id) = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        }
        Err(why) => panic!("Couldn't get app info: {:?}", why)
    };

    new_database();

    client.with_framework(
        StandardFramework::new()
            .configure(|configuration| {
               configuration
                .with_whitespace(true)
                .ignore_webhooks(true)
                .case_insensitivity(true)
                .on_mention(Some(bot_id))
                .owners(owners)
                .dynamic_prefix(|_, message| {
                    let default = "yabe".to_string();

                    if dotenv!("PROD") == "1" { return Some("yabedev".to_string());};

                    if let Some(guild_id) = message.guild_id {
                        Some(get_prefix(guild_id).map_or_else(|_| default, |prefix| prefix))
                    } else {
                        Some(default)
                    }
                })
            })
            .on_dispatch_error(|context, message, err| match err {
                DispatchError::Ratelimited(secs) => {
                    let _ = message.channel_id.say(
                        &context,
                        format!(
                            "You have hit a ratelimit bucket on Discord's end. Please try this \
                            again in {} seconds.",
                            secs
                        )
                    );
                }
                DispatchError::OnlyForOwners => {
                    let _ = message.channel_id.say(&context, "This command is not available for non-owners of Yabe.");
                }
                DispatchError::TooManyArguments { max, given } => {
                    let _ = message.channel_id.send_message(&context, |message| {
                        message.embed(|embed| {
                            embed.title("Too many arguments.");
                            embed.description(format!(
                                "That command requires less arguments! You provided {} arguments, but only {} were needed.",
                                max, given
                            ))
                        })
                    });
                }
                DispatchError::NotEnoughArguments { min, given } => {
                    let _ = message.channel_id.send_message(&context, |message| {
                        message.embed(|embed| {
                            embed.title("Not enough arguments.");
                            embed.description(format!(
                                "That command requires more arguments! You provided {} arguments, but {} were needed.",
                                min, given
                            ))
                        })
                    });
                }
                DispatchError::IgnoredBot => {}
                _ => error!("Dispatch Error: {} Failed: {:?}", message.content, err)
            })
            .after(|context, message, command_name, err| {
                if let Err(e) = err {
                    let _ = message.channel_id.say(&context, "Something went wrong running that command. Try again?");
                    error!("Encountered issue while running {} command\nUser: {}\nE: {:?}", command_name, message.author.tag(), e);
                }
            })
            .help(&HELP)
            .group(&FUN_GROUP)

    );

    if let Err(e) = client.start_autosharded() {
        error!("Could not run the client: {:?}", e);
    }
}