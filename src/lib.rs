use serenity::framework::standard::macros::group;
use serenity::prelude::EventHandler;

mod agenda;
mod arglexer;
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
pub const PREFIX: &str = "!";

#[group]
#[commands(help, minutes, poll, resource, agenda)]
pub struct General;
pub struct Handler;

impl EventHandler for Handler {}
