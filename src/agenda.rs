use chrono::prelude::Local;
use serde::{Deserialize, Serialize};
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::http::AttachmentType;
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use std::path::Path;
use std::{fmt, fs};

#[derive(Serialize, Deserialize, Default)]
struct Agenda {
    items: Vec<(String, String)>,
}

impl Agenda {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn save(&self, filename: &str) -> std::io::Result<()> {
        let path = Path::new(&filename);
        fs::write(&path, serde_json::to_string(&self)?)
    }

    fn push(&mut self, author: String, item: String) {
        self.items.push((author, item));
    }
}

impl From<&str> for Agenda {
    fn from(filename: &str) -> Self {
        let data = fs::read_to_string(filename).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_default()
    }
}

impl fmt::Display for Agenda {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let representation: String = self
            .items
            .iter()
            .map(|(p, i)| format!("\n** - {}**\t\t{}", p.replace("*", ""), i,))
            .collect::<String>();
        write!(f, "{}", representation)
    }
}

const ACKNOWLEDGED: char = '✅';
const FAILURE: char = '❌';

/// Controls the agenda of the next team meeting
///
/// Given the command `!resource <sub-command> <args> `, this handler will
/// perform one of the following operations:
///
/// `add <item>` Adds <item> to the agenda of the next meeting
/// `show`       Displays the full agenda in a message to the `meetings` channel
/// `export`     Exports the full agenda as a markdown file
/// `clear`      Clears the agenda
///
/// Because this command requires a level of state (preserving the agenda
/// between commands), mutual exclusion and lazy-static evaluation are utilised
/// to minimise the need for unsafe code
///
/// An acknowledgement reaction is used to relay feedback to the end user if
/// the sub-command does not cause an immediate result (e.g. add, clear). A
/// failure reaction is used to show that the sub-command was not recognised.
#[command]
async fn agenda(context: &Context, msg: &Message) -> CommandResult {
    // Collect the sub-command and arguments
    let args: Vec<&str> = msg.content.split(' ').skip(1).collect();
    tracing::info!(?args, "Executing an 'agenda' command");

    match args[0] {
        "add" | "append" | "push" => {
            // Load the agenda and add the new item
            let mut agenda: Agenda = Agenda::from("agenda.json");
            agenda.push(msg.author.name.clone(), args[1..].join(" "));
            agenda.save("agenda.json")?;

            // Relay feedback to the user
            msg.react(&context, ACKNOWLEDGED).await?;
        }
        "show" | "view" => {
            // Identify the target `meetings` channel
            let meetings_channel: ChannelId = msg
                .guild_id
                .ok_or("Message occurred outside of a Guild environment.")?
                .channels(&context)
                .await?
                .values()
                .find(|x| x.name == "meetings")
                .ok_or("Failed to find a channel with the name: 'meetings'")?
                .id;

            // Send the agenda to the `meetings` channel
            meetings_channel
                .send_message(&context, |m| {
                    let agenda: Agenda = Agenda::from("agenda.json");

                    if agenda.is_empty() {
                        m.content("**No items recorded** - sorry about that")
                    } else {
                        m.content(format!(
                            "**Meeting Agenda {}**{}",
                            Local::now().format("%Y-%m-%d"),
                            agenda
                        ))
                    }
                })
                .await?;
        }
        "export" => {
            // Translate the agenda to a markdown format
            let today = Local::now().format("%Y-%m-%d");
            let formatted_agenda = format!(
                "# Meeting Agenda {}{}",
                Local::now().format("%Y-%m-%d"),
                Agenda::from("agenda.json")
            );

            // Create the agenda file
            let filename = format!("agenda-{}.md", today);
            let path = Path::new(&filename);

            // Write the formatted agenda to the file
            fs::write(&path, formatted_agenda)?;

            // Send the file as an attachment to the `msg` source channel
            msg.channel_id
                .send_files(&context, vec![AttachmentType::Path(path)], |m| {
                    m.content(format!("**Meeting Agenda {}\n**", today))
                })
                .await?;

            // Delete the file once it has been sent
            fs::remove_file(&path)?;
        }
        "clear" | "new" => {
            // Log the event
            tracing::info!(user = %msg.author.name, "Cleared the agenda");

            // Replace the agenda with a default copy
            Agenda::new().save("agenda.json")?;

            // Relay feedback to the user
            msg.react(&context, ACKNOWLEDGED).await?;
        }
        _ => {
            // Relay the failure of the sub-command to the user
            msg.react(&context, FAILURE).await?;
            tracing::debug!(user = %msg.author.name, command = %args[0], "Unrecognised sub-command for 'agenda'");
        }
    }

    Ok(())
}
