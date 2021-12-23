use std::fs;
use std::path::Path;

use chrono::NaiveDate;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::http::AttachmentType;
use serenity::model::channel::Message;

/// Returns all messages from a given day in a given channel.
///
/// Given the command `!minutes <date>`, where <date> follows the format d/m/Y, this handler will
/// collect the messages sent on that date and format them into a structured Markdown document,
/// before sending it back to the caller.
#[command]
async fn minutes(context: &Context, msg: &Message) -> CommandResult {
    let args: Vec<&str> = msg.content.split(' ').skip(1).collect();
    tracing::info!(?args, "Executing a 'minutes' command");

    let day: NaiveDate = NaiveDate::parse_from_str(
        args.get(0).ok_or("Insufficient arguments provided.")?,
        "%d/%m/%Y",
    )?;
    tracing::info!(?day, "Parsed a date from the message");

    let messages: Vec<Message> = msg.channel_id.messages(&context, |b| b.limit(1000)).await?;
    tracing::debug!(count = %messages.len(), "Queried some messages from the API");

    let relevant: String = messages
        .iter()
        .filter(|x| x.timestamp.naive_local().date() == day)
        .map(|x| {
            format!(
                "\n{}\t**{}**:\t{}\n",
                x.timestamp.time().format("%H:%M").to_string(),
                x.author.name.replace("*", ""),
                x.content
            )
        })
        .rev()
        .collect::<String>();

    let formatted_minutes = format!("# Meeting minutes for {} \n{}", day, relevant);
    let filename = format!("minutes-{}.md", day.format("%Y-%m-%d"));
    let path = Path::new(&filename);

    // Write the formatted minutes to the file
    fs::write(&path, formatted_minutes)?;
    tracing::debug!(path = %path.display(), "Wrote some minutes to disk");

    msg.channel_id
        .send_files(&context, vec![AttachmentType::Path(path)], |m| {
            m.content(format!("Meeting minutes for {}", day))
        })
        .await?;

    // Delete the file once it has been sent
    fs::remove_file(&path)?;
    tracing::debug!(path = %path.display(), "Deleted a file from disk after sending it");

    Ok(())
}
