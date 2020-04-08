#![allow(unused_imports, unused_variables, unused_mut, unused_attributes, unused_parens, irrefutable_let_patterns, unused_must_use,)]
#![feature(type_ascription, box_into_pin)]

mod util;
mod listeners;
mod commands;

// command imports
use commands::fun::fun::*;
use commands::util::help::*;

use serenity::{
    http::Http,
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
    prelude::*
};

use tokio::sync::Mutex;

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
#[commands(roll, owofy, flip)]
struct Fun;

#[tokio::main]
pub async fn main() {
    let token = dotenv!("TOKEN");

    new_database();

    let http = Http::new_with_token(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        }
        Err(why) => panic!("Couldn't get app info: {:?}", why)
    };


    let framework = StandardFramework::new()
    .configure(|configuration| {
        configuration
        .with_whitespace(true)
        .ignore_webhooks(true)
        .case_insensitivity(true)
        .on_mention(Some(bot_id))
        .owners(owners)
        .dynamic_prefix(|_, message| Box::pin(async move {
            let default = "yabe".to_string();

            // if dotenv!("PROD") == "1" { "yabedev".to_string() };

            if dotenv!("PROD") == "0" {
                return Some("yabedev".to_string());
            }

            if let guild_id = message.guild_id {
                Some(get_prefix(guild_id.unwrap()).map_or_else(|_| default, |prefix| prefix))
            } else {
                Some(default)
            }
        }))
    })
    .on_dispatch_error(|context, message, err| Box::pin(async move { match err {
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
    }}))
    .after(|context, message, command_name, err| Box::pin(async move {
        if let Err(e) = err {
            let _ = message.channel_id.say(&context, "Something went wrong running that command. Try again?");
            error!("Encountered issue while running {} command\nUser: {}\nE: {:?}", command_name, message.author.tag(), e);
        }
    }))
    .help(&HELP)
    .group(&FUN_GROUP);


    let mut client = Client::new_with_framework(&token, Handler, framework)
    .await
    .expect("Coult not create Discord Client!");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    if let Err(e) = client.start_shards(1).await {
        error!("Could not run the client: {:?}", e);
    }
}

#[cfg(test)]
mod tests {

    use rand::Rng;

    #[test]
    fn gen_range() {
        for _ in 0..10 {
            let random = (rand::thread_rng().gen_range(1, 3) as f64).floor();
            println!("{}", random);
        }
    }
}
