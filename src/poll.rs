use std::str::FromStr;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::{Message, ReactionType};
use serenity::utils::Colour;

/// Unicode encodings for the emojis 1-9 to react with on poll messages.
const REACTIONS: [&str; 9] = [
    "\u{31}\u{FE0F}\u{20E3}",
    "\u{32}\u{FE0F}\u{20E3}",
    "\u{33}\u{FE0F}\u{20E3}",
    "\u{34}\u{FE0F}\u{20E3}",
    "\u{35}\u{FE0F}\u{20E3}",
    "\u{36}\u{FE0F}\u{20E3}",
    "\u{37}\u{FE0F}\u{20E3}",
    "\u{38}\u{FE0F}\u{20E3}",
    "\u{39}\u{FE0F}\u{20E3}",
];

/// Creates a poll message and sends it to the user.
///
/// Given a command such as `!poll Name Option1 Option2 Option3`, this will be called with the
/// arguments `[Name, Option1, Option2, Option3]`, allowing the bot to format the poll and send it.
///
/// Poll messages use an embedded message for nicer formatting, and add reactions for each option
/// that the poll provides.
#[command]
async fn poll(context: &Context, msg: &Message, args: Args) -> CommandResult {
    let args: Vec<_> = args.raw_quoted().collect();

    tracing::info!(?args, user = %msg.author.id, "Executing a 'poll' command");

    let (title, options) = args
        .split_first()
        .ok_or("Invalid number of arguments to !poll")?;

    tracing::info!(%title, ?options, "Parsed some arguments to the command");

    let formatted_options = options
        .iter()
        .zip(REACTIONS.iter())
        .map(|(o, r)| format!("{} `{}`", r, o))
        .collect::<Vec<String>>()
        .join("\n");

    msg.channel_id
        .send_message(&context, |m| {
            m.embed(|e| {
                e.title(title.to_uppercase())
                    .description(formatted_options)
                    .colour(Colour::from_rgb(0, 106, 176))
            })
            .reactions(
                REACTIONS
                    .iter()
                    .take(options.len())
                    .map(|s| ReactionType::from_str(s).unwrap()),
            )
        })
        .await?;

    tracing::info!("Responded with a formatted poll message");

    Ok(())
}
