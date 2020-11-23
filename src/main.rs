use twitchchat::PrivmsgExt as _;

use twitch_ranos::{
    Args,
    Bot,
    include::{channels_to_join, get_user_config},
    dice::{Dice},
    PREFIX
};

fn main() -> Result<(), String> {
    let user_config = get_user_config()?;
    let channels = channels_to_join()?;
    let start = std::time::Instant::now();

    let mut bot = Bot::default()
        .with_command(format!("{}roll", PREFIX), move |args: Args| {
            let dice: Vec<_> = args.params.iter().map(|&s| Dice::new(s)).take_while(|d| d.is_some()).map(|d| d.unwrap()).collect();
            let output = if dice.len() == args.params.len() {
                let mut out = String::new();
                let mut rolls: Vec<usize> = Vec::new();
                for d in &dice {
                    let roll = d.roll();
                    for r in &roll {
                        out = format!("{} {}, ", out, r);
                    }
                    rolls.extend(roll.iter());
                }
                out.pop();
                out.pop();
                format!("{} with a total sum of {}", out, rolls.iter().sum::<usize>())
            } else {
                "Error in given dice format. Please ensure that you pass dice configuration in \"xdy\" format, e.g. 1d20, 2d6, etc.".into()
            };
            args.writer.say(args.msg, &output).unwrap();
        })
        .with_command(format!("{}time", PREFIX), move |args: Args| {
            let output = if args.params.len() == 0 {
                format!("The current UTC time is {}", time::OffsetDateTime::now_utc().format("%F %T"))
            } else {
                let mut out = String::new();
                for param in args.params {
                    let time = time::OffsetDateTime::now_utc().to_offset(
                        if let Ok(offset) = time::UtcOffset::parse(param, "%z") {
                            offset
                        } else {
                            continue;
                        }
                    );
                    out = format!("{}\tThe current time at UTC{} is {}.", out, time.offset().format("%z"), time.format("%F %T"));
                }
                out
            };
            args.writer.say(args.msg, &output).unwrap();
        })
        .with_command(format!("{}uptime", PREFIX), move |args: Args| {
            let output = format!("its been running for {:.2?}", start.elapsed());
            args.writer.say(args.msg, &output).unwrap();
        })
        .with_command(format!("{}ping", PREFIX), move |args: Args| {
            let output = format!("pong!");
            args.writer.say(args.msg, &output).unwrap();
        })
        .with_command(format!("{}quit", PREFIX), move |args: Args| {
            smol::block_on(async move {
                let output = format!("Bot shutting down");
                args.writer.say(args.msg, &output).unwrap();
                args.quit.notify().await
            });
        })
    ;

    smol::block_on(async move { bot.run(&user_config, &channels).await })
}
