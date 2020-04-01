use crate::listeners::events::{
    guild_create,
    message,
    ready,
};

use serenity::{
    client::{
        Context, EventHandler,
    },
    model::{
        gateway::Ready,
        guild::Guild,
        prelude::Message,
    }
};

pub struct Handler;
impl EventHandler for Handler {
    fn message(&self, context: Context, message: Message) {
        message::message(context, message);
    }

    fn ready(&self, context: Context, ready: Ready) {
        ready::ready(context, ready);
    }

    fn guild_create(&self, context: Context, guild: Guild, _is_new: bool) {
        guild_create::guild_create(context, guild);
    }
}