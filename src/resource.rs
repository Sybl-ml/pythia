use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;

/// Forwards a message to the `#resources` channel.
///
/// Given the command `!resource <message>`, this handler will forward <message> to the
/// `#resources` channel with a short preamble.
#[command]
fn resource(context: &mut Context, msg: &Message) -> CommandResult {
    let first_space = msg
        .content
        .chars()
        .position(|c| c == ' ')
        .ok_or("Command has no arguments")?;

    let resource = &msg.content[first_space + 1..];

    let resources_channel: ChannelId = msg
        .guild_id
        .ok_or("Message occurred outside of a Guild environment.")?
        .channels(&context)?
        .values()
        .find(|x| x.name == "resources")
        .ok_or("Failed to find a channel with the name: 'resources'")?
        .id;

    resources_channel.send_message(&context, |m| {
        m.content(format!(
            "**{}** submitted the following resource:\n> {}",
            msg.author.name.replace("*", ""),
            resource
        ))
    })?;

    Ok(())
}
