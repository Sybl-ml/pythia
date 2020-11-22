use serenity::client::Client;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;
use serenity::prelude::EventHandler;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

mod agenda;
mod help;
mod minutes;
mod poll;
mod resource;

use agenda::AGENDA_COMMAND;
use help::HELP_COMMAND;
use minutes::MINUTES_COMMAND;
use poll::POLL_COMMAND;
use resource::RESOURCE_COMMAND;

/// The prefix for commands such as `!poll`.
const PREFIX: &str = "!";

#[group]
#[commands(help, minutes, poll, resource, agenda)]
struct General;
struct Handler;

impl EventHandler for Handler {}

#[tokio::main]
async fn main() -> Result<()> {
    // Configure the logger
    pretty_env_logger::formatted_timed_builder().init();

    // Read the token file into environment variables
    dotenv::from_filename("token.env")?;
    let token = std::env::var("token")?;

    log::info!("Initialised Pythia with a token, beginning execution");

    // Start the client with the token and the handler struct
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(
            StandardFramework::new()
                .configure(|c| c.prefix(PREFIX))
                .group(&GENERAL_GROUP),
        )
        .await?;

    client.start().await?;

    Ok(())
}
