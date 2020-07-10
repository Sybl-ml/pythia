use chrono::prelude::Local;
use serde::{Deserialize, Serialize};
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::http::AttachmentType;
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Default)]
struct Agenda {
    items: Vec<String>,
}

impl Agenda {
    pub fn new() -> Agenda {
        return Agenda { items: Vec::new() };
    }

    pub fn from(filename: &str) -> Agenda {
        let data = fs::read_to_string(filename).unwrap_or_default();
        return serde_json::from_str(&data).unwrap_or_default();
    }

    fn save(&self, filename: &str) -> std::io::Result<()> {
        let path = Path::new(&filename);
        return fs::write(&path, serde_json::to_string(&self)?);
    }
}

const ACKNOWLEDGED: &str = "✅";
const FAILURE: &str = "❌";

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
fn agenda(context: &mut Context, msg: &Message) -> CommandResult {
    // Collect the sub-command and arguments
    let args: Vec<&str> = msg.content.split(' ').skip(1).collect();

    match args[0] {
        "add" => {
            // Load the agenda and add the new item
            let mut agenda: Agenda = Agenda::from("agenda.json");
            agenda.items.push(format!(
                "\n** - {}**\t\t{}",
                msg.author.name.replace("*", ""),
                args[1..].join(" ")
            ));
            agenda.save("agenda.json")?;

            // Relay feedback to the user
            msg.react(&context, ACKNOWLEDGED)?;
        }
        "show" => {
            // Identify the target `meetings` channel
            let meetings_channel: ChannelId = msg
                .guild_id
                .ok_or("Message occurred outside of a Guild environment.")?
                .channels(&context)?
                .values()
                .find(|x| x.name == "meetings")
                .ok_or("Failed to find a channel with the name: 'meetings'")?
                .id;

            // Send the agenda to the `meetings` channel
            meetings_channel.send_message(&context, |m| {
                let agenda: Agenda = Agenda::from("agenda.json");
                if agenda.items.len() == 0 {
                    m.content(format!("**No items recorded** - sorry about that",))
                } else {
                    m.content(format!(
                        "**Meeting Agenda {}**{}",
                        Local::now().format("%Y-%m-%d"),
                        agenda.items.join("")
                    ))
                }
            })?;
        }
        "export" => {
            // Translate the agenda to a markdown format
            let today = Local::now().format("%Y-%m-%d");
            let formatted_agenda = format!(
                "# Meeting Agenda {}{}",
                Local::now().format("%Y-%m-%d"),
                Agenda::from("agenda.json").items.join("")
            );

            // Create the agenda file
            let filename = format!("agenda-{}.md", today);
            let path = Path::new(&filename);

            // Write the formatted agenda to the file
            fs::write(&path, formatted_agenda)?;

            // Send the file as an attachment to the `msg` source channel
            msg.channel_id
                .send_files(&context, vec![AttachmentType::Path(&path)], |m| {
                    m.content(format!("**Meeting Agenda {}\n**", today))
                })?;

            // Delete the file once it has been sent
            fs::remove_file(&path)?;
        }
        "clear" => {
            // Replace the agenda with a default copy
            Agenda::new().save("agenda.json")?;

            // Relay feedback to the user
            msg.react(&context, ACKNOWLEDGED)?;
        }
        _ => {
            // Relay the failure of the sub-command to the user
            msg.react(&context, FAILURE)?;
        }
    }

    Ok(())
}
