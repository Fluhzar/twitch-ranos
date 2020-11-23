# twitch-ranos

NOTE: This bot is not ready for regular use and is at best in the experimentation stages.

Some things that need implementation:

* Credential level checks, e.g. it shouldn't be possible for any random user to type `!quit` and shutdown the bot.
* More usable commands other than gimmicks.

## Usage

Command prefix: `!` - This can be customized in `lib.rs` by changing the value of the `PREFIX` const.

The bot should be run with the following environment variables set:

* `TWITCH_USERNAME`: The username of the bot (e.g. `TWITCH_USERNAME=ranosbot`).
* `TWITCH_TOKEN`: The token to authenticate the bot with Twitch's servers. Generate with [this link][0]. It should look something like `TWITCH_TOKEN=oauth:big_long_string_of_letters_and_numbers`.
* `TWITCH_CHANNELS`: Comma separated list of channels this bot should listen to. Example: `TWITCH_CHANNELS=fluhzar,channel2,channel3`

### Commands

* `ping`: replies with `pong!`.
* `quit`: shuts down the bot. There currently is no protection on this command, anyone can enter `!quit` in the chat.
* `roll`: takes parameters in standard dice format (e.g. 1d6, 3d20, etc.), rolls the corresponding dice and reports the individual roll values as well as the sum of the values.
* `time`: prints the current time. If no additional parameters are given, it displays the time at UTC+0000, otherwise it displays the time at the given UTC offset (e.g. `-0800` for US West Coast PST timezone). Multiple parameters can be given to display the current time in multiple timezones.
* `uptime`: responds with the number of seconds the bot has been up.

[0]: https://twitchapps.com/tmi/
