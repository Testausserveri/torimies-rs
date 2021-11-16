# torimies-rs

## How the bot works?

The bot works by making requests to the undocumented ~~(and very bad)~~ tori.fi api endpoint.
The users can add and remove new vahti-entries for themselves. Those are stored in the sqlite-database of the bot.
The bot then goes through all of the defined vahtis in the database and sends the updates to the users accordingly.

## How to host?

Make sure your `.env` includes all the necessary stuff:
* `DATABASE_URL=sqlite:database.sqlite` (unless you change the location)
* `DISCORD_TOKEN=YourToken`
* `APPLICATION_ID=YourAppID`
* `UPDATE_INTERVAL=time_in_seconds` (defaults to 60)

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


Please mind that the bot is still considered to be WIP.
We will gladly accept any feedback/feature requirests :), just file an [issue](https://github.com/lajp/torimies-rs/issues) and we'll look into it
