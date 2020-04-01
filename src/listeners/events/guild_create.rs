use crate::util::db::get_database;
use log::info;
use serenity::{client::Context, model::guild::Guild};

#[macro_use]
use dotenv_codegen;

pub fn guild_create(_context: Context, guild: Guild) {
    let conn = match get_database() {
        Ok(connection) => connection,
        Err(_) => return
    };

    let prefix = "yabe".to_string();

    let guild_id = guild.id.as_u64().to_string();

    let _ = conn.execute(
        "INSERT OR IGNORE INTO guilds(guild_id, prefix) values(?1, ?2);",
        &[&guild_id, &prefix]
    );
}