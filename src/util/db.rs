use log::error;
use rusqlite::{Connection, NO_PARAMS};
use serenity::model::prelude::{GuildId, UserId};

use std::{
    error::Error,
    fs::File,
    path::Path,
};

pub fn new_database() {
    let db_path = Path::new("yabe.db");

    if !db_path.exists() {
        match File::create(&db_path) {
            Ok(_) => (),
            Err(e) => error!("Something went wrong creating the sqlite file.\n{}", e)
        }
    }

    // creating tables etc.
    if let Ok(connection) = Connection::open(&db_path) {
        match connection.execute(
            "CREATE TABLE IF NOT EXISTS guilds (
                guild_id TEXT PRIMARY KEY,
                prefix TEXT NOT NULL
            );",
            NO_PARAMS
        ) {
            Ok(_) => (),
            Err(e) => error!("{}", e)
        };
    } else {
        error!("Connection to database failed!")
    };
}

pub fn get_database() -> Result<Connection, Box<dyn Error>> {
    let database_file = Path::new("yabe.db");
    Ok(Connection::open(database_file)?)
}

pub fn get_prefix(guild_id: GuildId) -> Result<String, Box<dyn Error>> {
    let conn = get_database()?;
    let mut statement = conn.prepare("SELECT prefix FROM guilds WHERE guild_id = ?1")?;
    let mut rows = statement.query(&[&guild_id.as_u64().to_string()])?;
    Ok(rows.next()?.ok_or("Guild not found.")?.get(0)?)
}