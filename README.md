# torimies-rs

## How the bot works?

The bot works by making requests to the undocumented ~~(and very bad)~~ tori.fi api endpoint.
The users can add and remove new vahti-entries for themselves. Those are stored in the sqlite-database of the bot.
The bot then goes through all of the defined vahtis in the database and sends the updates to the users accordingly.

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

Before running you need to setup the sqlite-database

I do this with the `sqlx-cli` tool.

The tool is installed with `cargo install sqlx-cli`

After that you can just run the `reset_db.sh` script
after which your database should be good to go.

The binary builds include a pre-initialize database :)

### Running torimies-rs

**If you are building from source**, just do `cargo run --release`
in the root of the repository

**If you are are using a binary build**, just do `./torimies-rs`

## Using the bot

The bot has two main commands implemented as application commands (slash-commands)
and those are:
* `/vahti url` Add a new vahti with the specified url
* `/poistavahti url` Remove the vahti with the specified url

One additional owner-commmand is also included:
* `!update_all_vahtis` to well... update all vahtis :D.


Please keep in mind that the bot is still considered to be WIP.
We will gladly accept any feedback/feature requests :), just file an [issue](https://github.com/Testausserveri/torimies-rs/issues) and we'll look into it.
