use serenity::client::Client;
use serenity::framework::standard::StandardFramework;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Sets up the logging for the application.
pub fn setup() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info,pythia=debug")
    }

    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}

#[tokio::main]
async fn main() -> Result<()> {
    // Configure the logger
    setup();

    // Read the token file into environment variables
    dotenv::from_filename("token.env").ok();
    let token = std::env::var("token")?;

    tracing::info!("Initialised Pythia with a token, beginning execution");

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
