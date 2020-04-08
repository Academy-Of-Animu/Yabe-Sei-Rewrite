use serenity::{
    framework::standard::{
        Args, CommandResult,
        macros::{command, group},
    },
    prelude::*,
    model::{prelude::*, guild::Ban},
    utils::parse_mention,
};


use std::{
    time::Duration,
};

#[group("collector")]
#[commands(ban)]
struct Collector;

#[command]
#[description = "The `ban` command requires `Ban Members` permission. It allows you to ban a specified user."]
#[usage = "<@user> <reason>"]
pub async fn ban(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    if args.rest().is_empty() {
        message.channel_id.send_message(&context, |msg| {
            msg.embed(|e| {
                e.description("Please mention a user to ban!");
                e.color(0x1355A4)
            })
        }).await;
        return Ok(());
    }


    let args = format!("{}", args.rest()).to_string();
    let args:  Vec<&str> = args.split_whitespace().collect();

    let member_id = parse_mention(args[0]).unwrap();
    let reason = args[1..].join(" ");

    let ask_to_confirm_message = format!("Are you sure you want to ban <@{:?}> for {}? \nReply with \"yes\" or \"no\" in the next 10 seconds", member_id, reason);
    message.channel_id.send_message(&context, |msg|{
        msg.content(ask_to_confirm_message)
    }).await;


    if let Some(answer) = message.author.await_reply(&context).timeout(Duration::from_secs(10)).await {

        if answer.content.to_lowercase() == "yes" {
            //ban user
            message.channel_id.send_message(&context, |msg| {
                msg.embed(|e| {
                    e.title(format!("User was banned by {}", message.author.name));
                    e.description(format!("User <@{:?}> was banned for:\n{}", member_id, reason));
                    e.color(0x1355A4)
                })
            }).await;
        } else if answer.content.to_lowercase() == "no" {
            let _ = message.reply(&context, "Aborting ban...").await;
        }

    } else {
        let _ =  message.reply(&context, "No answer within 10 seconds, aborting ban.").await;
    }

    Ok(())
}
