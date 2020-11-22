use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

const DOCUMENTATION: &str = " \
My name is **Pythia**! I'm here to help you manage your project in Discord!
`usage: !<command> [<args>]`

Manage the **agenda** of a future meeting:
```
!agenda add <item>    add an <item> to the current agenda
!agenda show          send the agenda in the 'meetings' channel
!agenda export        export the agenda as a markdown file
!agenda clear         clear the agenda
```
Collate meeting **minutes** for a given day:
```
!minutes <date>       export the messages sent in the 'meetings' channel on
                      the date <date> (D/M/Y) as a formatted markdown file
```
Create a **poll** message:
```
!poll <title> <args>  creates a poll message with title <title> and options
                      <args>, up to a maximum of 10 arguments
```
Record a useful **resource** for future reference:
```
!resource <item>      records the <item> in the 'resources' channel to make
                      sure it doesn't get lost in discussion
```";

/// Given the command `!help`, this handler will send a useful, formatted
/// help message explaining Pythia commands.
#[command]
async fn help(context: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .send_message(&context, |m| m.content(DOCUMENTATION))
        .await?;

    Ok(())
}
