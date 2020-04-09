use serenity::{
    client::Context,
    framework::standard::{
        help_commands,
        macros::help,
        Args,
        HelpOptions,
        CommandGroup,
        CommandResult,
    },
    model::prelude::{
        Message,
        UserId,
    },
    utils::Colour,
};

use std::collections::HashSet;

#[help]
#[max_levenshtein_distance(2)]
#[no_help_available_text("There is no command available with that name. Please try again.")]
pub async fn help(context: &mut Context, message: &Message, args: Args, options: &'static HelpOptions, command_groups: &[&'static CommandGroup], bot_owners: HashSet<UserId>) -> CommandResult {
    let mut custom_help_options = options.clone();
    custom_help_options.embed_success_colour = Colour::from(0x1355A4);

    help_commands::with_embeds(context, message, args, &custom_help_options, command_groups, bot_owners).await
}