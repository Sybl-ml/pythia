use serenity::client::Client;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;
use serenity::prelude::EventHandler;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

mod agenda;
mod minutes;
mod poll;
mod resource;

use agenda::AGENDA_COMMAND;
use minutes::MINUTES_COMMAND;
use poll::POLL_COMMAND;
use resource::RESOURCE_COMMAND;

/// The prefix for commands such as `!poll`.
const PREFIX: &str = "!";

#[group]
#[commands(minutes, poll, resource, agenda)]
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
