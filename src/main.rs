use dotenv;
use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::prelude::{Context, EventHandler};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, context: Context, msg: Message) {
        if msg.content == "!ping" {
            let _ = msg.channel_id.say(&context, "Pong!");
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
