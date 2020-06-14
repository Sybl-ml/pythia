use chrono::NaiveDate;
use serenity::client::Client;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use serenity::prelude::{Context, EventHandler};
use serenity::utils::Colour;

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
/// The ChannelId associated with the server's `#resources` channel.
const RESOURCES_CHANNEL: ChannelId = ChannelId(719260683782455377);

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

/// Returns all messages from a given day in a given channel.
///
/// Given the command `!minutes <date>`, where <date> follows the format d/m/Y, this handler will
/// collect the messages sent on that date and format them into a structured Markdown document,
/// before sending it back to the caller.
#[command]
fn minutes(context: &mut Context, msg: &Message) -> CommandResult {
    let args: Vec<&str> = msg.content.split(" ").skip(1).collect();
    let day: NaiveDate = NaiveDate::parse_from_str(args[0], "%d/%m/%Y").unwrap();

    let messages: Vec<Message> = msg
        .channel_id
        .messages(&context, |b| b.limit(1000))
        .unwrap();

    let relevant: String = messages
        .iter()
        .filter(|x| x.timestamp.naive_local().date() == day)
        .map(|x| {
            format!(
                "\n{}\t**{}**:\t*{}*\n",
                x.timestamp.time().format("%H:%M").to_string(),
                x.author.name.replace("*", ""),
                x.content.replace("*", "")
            )
        })
        .rev()
        .collect::<String>();

    let _sent_message = msg
        .channel_id
        .send_message(&context, |m| {
            m.content(format!(
                "**Meeting minutes for {}** \n{}",
                args[0], relevant
            ))
        })
        .unwrap();

    Ok(())
}

/// Forwards a message to the `#resources` channel.
///
/// Given the command `!resource <message>`, this handler will forward <message> to the
/// `#resources` channel with a short preamble.
#[command]
fn resource(context: &mut Context, msg: &Message) -> CommandResult {
    let resource: String = msg
        .content
        .chars()
        .skip_while(|c| c != &' ')
        .collect::<String>();

    let _sent_message = RESOURCES_CHANNEL
        .send_message(&context, |m| {
            m.content(format!(
                "**{}** submitted the following resource:\n > {}",
                msg.author.name.replace("*", ""),
                resource
            ))
        })
        .unwrap();

    Ok(())
}

#[group]
#[commands(poll, minutes, resource)]
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
