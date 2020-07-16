# Pythia

Pythia is a Discord bot written in Rust to add functionality to our Discord
server while providing a way to learn Rust.

It is named after [Pythia](https://en.wikipedia.org/wiki/Pythia), who was the
[Oracle](https://en.wikipedia.org/wiki/Oracle) of
[Delphi](https://en.wikipedia.org/wiki/Delphi). It aims to provide
functionality such as poll creation, meeting minute recording, agenda
management and resource cataloguing.

## Usage

Pythia uses the `!` command prefix and will not respond to any other messages.

`usage: !<command> [<args>]`

Pythia can perform many tasks to assist project management in a Discord server:

 - Manage the **agenda** of a future meeting:

		!agenda add <item>    add an <item> to the current agenda
		!agenda show          send the agenda in the 'meetings' channel
		!agenda export        export the agenda as a markdown file
		!agenda clear         clear the agenda

 - Collate meeting **minutes** for a given day:

        !minutes <date>       export the messages sent in the 'meetings' channel on
                              the date <date> (D/M/Y) as a formatted markdown file

 - Create a **poll** message:

        !poll <title> <args>  creates a poll message with title <title> and options
                              <args>, up to a maximum of 10 arguments

 - Record a useful **resource** for future reference:

        !resource <item>      records the <item> in the 'resources' channel to make
                              sure it doesn't get lost in discussion

This information can also be retrieved by sending the `!help` command to Pythia.

## Development

The bot can be built by cloning the repository and building it using `cargo
build`, which will install all the required dependencies.

It expects the Discord token to be found in a file called `token.env`, which
should be in the root directory of the project. This should contain a single
line in the form `token=<token>`.

### Discord Setup

Tokens can be generated on the Discord website using the following
instructions:

- Open the [Discord Developer Portal](https://discord.com/developers/applications)
- Create a new application (FT2 Helper)
- Navigate to the `Bot` tab and add a bot, giving it a username
- Copy the token under the username into the `token.env` file

Now that the bot exists, you can add it to a server to begin testing it. The
easiest way to do this is to create a private server for you and the bot before
you let it loose publicly.

- Navigate to the `OAuth2` tab
- Under `Scopes`, check the `bot` tickbox
- Enable the relevant permissions under `Bot Permissions`
	- Currently this is just `Send Messages` and `Add Reactions`
- Open the resulting URL to choose the server to add it to

After these steps, you should be able to run the bot and send a message such as
`!poll Title Option1 Option2` in the server, with the bot responding with a
formatted poll and reacting with options.
