# torimies-rs

## How the bot works

Users of the bot can create and remove vahti-entries that they have made. Vahti-entries are stored in the sqlite-database of the bot.
The vahtis in the database are periodically checked for new matches on the tori.fi site using an undocumented api endpoint, and new matching listings are then sent to the vahti's creator.

![](./media/demo.png)

## Hosting the bot
If you do not have a discord application ready create one [here](https://discord.com/developers/applications). Create a bot user for the application if one doesn't already exist.

When you have your discord application ready, visit the following link to generate an invite link: `https://discord.com/developers/applications/YourAppID/oauth2/url-generator`.
Replace "YourAppID" with the application id of your application.

The discord application invite link used should have the following scopes:
 - `bot` - required for the invite link to be a bot-invite link
 - `applications.commands` - required for the bot commands to be usable

Make sure to create the `.env` file if it does not exist and ensure that it contains all the necessary variables:
* `DATABASE_URL=sqlite:database.sqlite` (or another location)
* `DISCORD_TOKEN=YourToken` (the token for your discord bot)
* `APPLICATION_ID=YourAppID` (the discord application id)

Optional variables:
* `UPDATE_INTERVAL=time_in_seconds` (the interval at which the bot updates vahtis, defaults to 60)

### Setting up the database

Before starting the bot you must setup the sqlite-database. This can be done with the `sqlx-cli` tool, which is used in these instructions.

`sqlx-cli` can be installed using `cargo install sqlx-cli`.

After installing the `sqlx-cli` tool the `reset_db.sh` script can be run
to automatically set up the database, deleting any existing database.

The binary builds include a pre-initialized database.

### Running torimies-rs

**If you are building from source** run `cargo run --release` in the root of the repository.

**If you are are using a binary build** run `./torimies-rs`.

## Using the bot

The bot has two main commands implemented as application commands (slash-commands)
and those are:
* `/vahti url` Adds a new vahti with the specified url
* `/poistavahti url` Removes the vahti with the specified url

One additional owner-restricted commmand is also included (this is not a slash-command):
* `!update_all_vahtis` immediately updates all vahtis


Please keep in mind that the bot is still considered to be WIP.
We will gladly accept any feedback/feature requests :), just file an [issue](https://github.com/Testausserveri/torimies-rs/issues) and we'll look into it.
