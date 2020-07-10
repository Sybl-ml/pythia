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
fn minutes(context: &mut Context, msg: &Message) -> CommandResult {
    let args: Vec<&str> = msg.content.split(' ').skip(1).collect();
    log::info!("Executing 'minutes' command with args: {:?}", args);

    let day: NaiveDate = NaiveDate::parse_from_str(
        args.get(0).ok_or("Insufficient arguments provided.")?,
        "%d/%m/%Y",
    )?;
    log::info!("Date was interpreted as: {}", day);

    let messages: Vec<Message> = msg.channel_id.messages(&context, |b| b.limit(1000))?;
    log::info!("Number of messages pulled from chat: {}", messages.len());

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
    log::info!("Wrote the minutes to: {}", &path.display());

    msg.channel_id
        .send_files(&context, vec![AttachmentType::Path(&path)], |m| {
            m.content(format!("Meeting minutes for {}", day))
        })?;

    // Delete the file once it has been sent
    fs::remove_file(&path)?;
    log::info!(
        "After sending, removed the file from disk: {}",
        &path.display()
    );

    Ok(())
}
