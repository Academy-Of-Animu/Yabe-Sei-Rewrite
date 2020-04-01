use crate::util::db::get_database;

use serenity::{
    client::Context,
    model::prelude::Message,
};

pub fn message(_context: Context, message: Message) {
    if message.author.bot { return }
}