use crate::listeners::events::{
    guild_create,
    message,
    ready,
};

use serenity::{
    async_trait,
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

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, message: Message) {
        message::message(context, message);
    }

    async fn ready(&self, context: Context, ready: Ready) {
        ready::ready(context, ready);
    }

    async fn guild_create(&self, context: Context, guild: Guild, _is_new: bool) {
        guild_create::guild_create(context, guild);
    }
}
