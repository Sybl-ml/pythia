use serenity::client::Client;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::{Context, EventHandler};
use serenity::utils::Colour;
use chrono::NaiveDate;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// The prefix for commands such as `!poll`.
const PREFIX: &str = "!";
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
fn poll(context: &mut Context, msg: &Message) -> CommandResult {
    let args: Vec<&str> = msg.content.split(" ").skip(1).collect();
    let (title, options) = args.split_first().unwrap();

    let formatted_options = options
        .iter()
        .zip(REACTIONS.iter())
        .map(|(o, r)| format!("{} `{}`", r, o))
        .collect::<Vec<String>>()
        .join("\n");

    let sent_message = msg
        .channel_id
        .send_message(&context, |m| {
            m.embed(|e| {
                e.title(title.to_uppercase())
                    .description(formatted_options)
                    .colour(Colour::from_rgb(0, 106, 176))
            })
        })
        .unwrap();

    for reaction in REACTIONS.iter().take(options.len()) {
        sent_message.react(&context, *reaction)?;
    }

    Ok(())
}

/// Returns all messages from a given day in a given channel
///
/// Given the command `!minutes Date`, where Date follows the format D/M/Y, 
/// this function will be called with the arguments `[Date]`.
///
/// Messages are now formatted to appear clear and readable in a discord 
/// channel. Future versions may also supply a markdown output.
#[command]
fn minutes(context: &mut Context, msg: &Message) -> CommandResult {
    let args: Vec<&str> = msg.content.split(" ").skip(1).collect();
    let day = NaiveDate::parse_from_str(args[0], "%d/%m/%Y").unwrap();

    let messages = msg
        .channel_id
        .messages(&context, |b| {
            b.limit(1000)
        })
        .unwrap();

    let relevant = messages.iter()
        .filter(|x| x.timestamp.naive_local().date() == day)
        .map(|x| format!(
            "\n{}\t**{}**:\t*{}*\n", 
            x.timestamp.time().format("%H:%M").to_string(),
            x.author.name.replace("*", ""), 
            x.content.replace("*", "")
        )).rev().collect::<String>();

    let _sent_message = msg
        .channel_id
        .send_message(&context, |m| {
            m.content(format!(
                "**Meeting minutes for {}** \n{}",
                args[0],
                relevant
            ))
        })
        .unwrap();

    Ok(())
}

#[group]
#[commands(poll)]
struct General;
struct Handler;

impl EventHandler for Handler {}

fn main() -> Result<()> {
    // Read the token file into environment variables
    dotenv::from_filename("token.env")?;
    let token = std::env::var("token")?;

    // Start the client with the token and the handler struct
    let mut client = Client::new(token, Handler)?;
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix(PREFIX))
            .group(&GENERAL_GROUP),
    );
    client.start()?;

    Ok(())
}
