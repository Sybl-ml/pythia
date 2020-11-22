use serenity::client::Client;
use serenity::framework::standard::StandardFramework;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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
        .event_handler(pythia::Handler)
        .framework(
            StandardFramework::new()
                .configure(|c| c.prefix(pythia::PREFIX))
                .group(&pythia::GENERAL_GROUP),
        )
        .await?;

    client.start().await?;

    Ok(())
}
