use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::prelude::{Context, EventHandler};
use serenity::utils::Colour;
use chrono::NaiveDate;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Handler;

/// The prefix for commands such as `!poll`.
const PREFIX: char = '!';
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
fn create_poll(context: &Context, msg: &Message, args: &[&str]) {
    let (title, options) = args.split_first().unwrap();

    let formatted_options = options
        .iter()
        .zip(REACTIONS.iter())
        .map(|(o, r)| format!("{} `{}`", r, o))
        .collect::<Vec<String>>()
        .join("\n");

    let sent_message = msg
        .channel_id
        .send_message(context, |m| {
            m.embed(|e| {
                e.title(title.to_uppercase())
                    .description(formatted_options)
                    .colour(Colour::from_rgb(0, 106, 176))
            })
        })
        .unwrap();

    for reaction in REACTIONS.iter().take(options.len()) {
        while let Err(_) = sent_message.react(context, *reaction) {}
    }
}

/// Returns all messages from a given day in a given channel
///
/// Given a command such as `!minutes Date`, where Date follows the format d/m/Y, 
/// this will be called with the arguments `[Date]`.
///
/// Messages are currently completely unformatted. Future releases should format
/// messages nicely into a markdown format, or similar.
fn get_minutes(context: &Context, msg: &Message, args: &[&str]) {
    let day = NaiveDate::parse_from_str(args[0], "%d/%m/%Y").unwrap();

    let messages = msg
        .channel_id
        .messages(context, |b| {
            b.limit(1000)
        })
        .unwrap();

    let relevant = messages.iter()
        .filter(|x| x.timestamp.naive_local().date() == day)
        .map(|x| format!(
            "\n**{}**:\t*{}*\n", 
            x.author.name.replace("*", ""), 
            x.content.replace("*", "")
        )).rev().collect::<String>();

    // TODO: format these messages nicely

    let _sent_message = msg
        .channel_id
        .send_message(context, |m| {
            m.content(format!(
                "# Meeting minutes for {} {}\n",
                args[0],
                relevant
            ))
        })
        .unwrap();

}

/// Parses a command from a message and dispatches to the correct handler.
///
/// Given a message that begins with PREFIX, this will split the command into tokens by spaces and
/// dispatch the arguments to the appropriate handler function. The command is interpreted as the
/// first element of the vector with the arguments as the remaining portion.
fn dispatch(context: &Context, msg: &Message) {
    let tokens: Vec<&str> = msg.content.split(' ').collect();
    let (command, args) = tokens.split_first().unwrap();

    match &command[1..] {
        "poll" => create_poll(context, msg, &args),
        "minutes" => get_minutes(context, msg, &args),
        // TODO: Respond with some kind of message or ignore this //
        _ => println!("Unknown command"),
    }
}

impl EventHandler for Handler {
    /// Registers a handler for a message being received.
    fn message(&self, context: Context, msg: Message) {
        // Check whether we have the correct prefix, otherwise ignore
        if let Some(first) = msg.content.chars().next() {
            if first == PREFIX {
                dispatch(&context, &msg);
            }
        }
    }
}

fn main() -> Result<()> {
    // Read the token file into environment variables
    dotenv::from_filename("token.env")?;
    let token = std::env::var("token")?;

    // Start the client with the token and the handler struct
    let mut client = Client::new(token, Handler)?;
    client.start()?;

    Ok(())
}
