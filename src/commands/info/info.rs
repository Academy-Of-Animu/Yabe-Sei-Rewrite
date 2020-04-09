use serenity::{
    client::Context,
    framework::standard::{
        macros::command,
        Args,
        CommandResult,
    },
    model::prelude::Message,
};

#[command]
#[description = "Displays info about Yabe Sei, and the creators of Yabe."]
pub async fn info(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    message.channel_id.send_message(&context, |msg| {
        msg.embed(|e| {
            e.description(
                "canarado - Lead Developer of Yabe Sei\n\n
                leet_hakker - co-developer of Yabe Sei\n\n
                Join our [Discord](https://discord.gg/bhZGHCm) to talk with the devs and give your thoughts on Yabe\n\n
                (These details are all according to the Yabe Rewrite project.)"
            );
            e.color(0x1355A4)
        })
    }).await;

    Ok(())
}