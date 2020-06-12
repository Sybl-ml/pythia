use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::prelude::{Context, EventHandler};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Handler;

const PREFIX: char = '!';
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
            m.embed(|e| e.title(title.to_uppercase()).description(formatted_options))
        })
        .unwrap();

    for reaction in REACTIONS.iter().take(options.len()) {
        while let Err(_) = sent_message.react(context, *reaction) {}
    }
}

fn dispatch(context: &Context, msg: &Message) {
    let tokens: Vec<&str> = msg.content.split(' ').collect();
    let (command, args) = tokens.split_first().unwrap();

    match &command[1..] {
        "poll" => create_poll(context, msg, &args),
        // TODO: Respond with some kind of message or ignore this //
        _ => println!("Unknown command"),
    }
}

impl EventHandler for Handler {
    fn message(&self, context: Context, msg: Message) {
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

    let mut client = Client::new(token, Handler)?;
    client.start()?;

    Ok(())
}
