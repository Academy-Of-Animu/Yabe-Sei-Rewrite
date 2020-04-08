use serenity::{
    client::Context,
    model::{
        gateway::{Activity, Ready},
        user::OnlineStatus
    }
};

use log::info;

pub async fn ready(context: Context, ready: Ready) {
    let application_info = context.http.get_current_application_info().await;

    info!("{} is logged into Discord", ready.user.tag());
    info!("Gateway version {}", ready.version);
    info!("Since readied, connected to {} guilds.", ready.guilds.len());

    context.set_presence(Some(Activity::playing("yabe help for info :)")), OnlineStatus::Online);
}