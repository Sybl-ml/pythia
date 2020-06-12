use dotenv;
use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::prelude::{Context, EventHandler};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, context: Context, msg: Message) {
        if msg.content == "whoami" {
            let private_channel_id = msg.author.create_dm_channel(&context).unwrap().id;
            let response = format!("You are {}#{}", msg.author.name, msg.author.discriminator);
            let private_message = private_channel_id.say(&context, response).unwrap();
            let reaction = private_message.react(&context, '\u{2764}');
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
