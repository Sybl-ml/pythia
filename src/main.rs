use fern::colors::{Color, ColoredLevelConfig};
use serenity::client::Client;
use serenity::framework::standard::StandardFramework;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Sets up the logging for the application.
///
/// Initialises a new instance of a [`fern`] logger, which displays the time and some coloured
/// output based on the level of the message. It also suppresses output from libraries unless they
/// are warnings or errors, and enables all log levels for the current binary.
pub fn setup_logger(lvl_for: &'static str) {
    let colours_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::BrightBlack);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{colours_line}[{date}][{target}][{level}]\x1B[0m {message}",
                colours_line = format_args!(
                    "\x1B[{}m",
                    colours_line.get_color(&record.level()).to_fg_str()
                ),
                date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                target = record.target(),
                level = record.level(),
                message = message,
            ));
        })
        .level(log::LevelFilter::Warn)
        .level_for(lvl_for, log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply()
        .expect("Failed to initialise the logger");
}

#[tokio::main]
async fn main() -> Result<()> {
    // Configure the logger
    setup_logger("pythia");

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
