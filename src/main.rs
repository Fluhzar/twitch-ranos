use twitchchat::PrivmsgExt as _;
use twitchchat::{
    messages::{Commands, Privmsg},
    runner::{AsyncRunner, NotifyHandle, Status},
    UserConfig,
};
use crate::include::{channels_to_join, get_user_config};
use std::collections::HashMap;

fn main() -> Result<(), String> {
    let user_config = get_user_config()?;
    let channels = channels_to_join()?;
    let start = std::time::Instant::now();

    let mut bot = Bot::default()
        .with_command("!hello", |args: Args| {
            let output = format!("hello {}!", args.msg.name());
            args.writer.reply(args.msg, &output).unwrap();
        })
        .with_command("!uptime", move |args: Args| {
            let output = format!("its been running for {:.2?}", start.elapsed());
            args.writer.say(args.msg, &output).unwrap();
        })
        .with_command("!quit", move |args: Args| {
            smol::block_on(async move {
                args.quit.notify().await
            });
        });

    smol::block_on(async move { bot.run(&user_config, &channels).await })
}

pub(crate) fn error_to_string<T>(res: Result<T, impl std::error::Error>) -> Result<T, String> {
    res.map_err(|e| e.to_string())
}

struct Args<'a, 'b: 'a> {
    msg: &'a Privmsg<'b>,
    writer: &'a mut twitchchat::Writer,
    quit: NotifyHandle,
}

trait Command: Send + Sync {
    fn handle(&mut self, args: Args<'_,'_>);
}

impl<F> Command for F
where
    F: Fn(Args<'_,'_>),
    F: Send + Sync,
{
    fn handle(&mut self, args: Args<'_,'_>) {
        (self)(args)
    }
}

#[derive(Default)]
struct Bot {
    commands: HashMap<String, Box<dyn Command>>,
}

impl Bot {
    fn with_command(mut self, name: impl Into<String>, cmd: impl Command + 'static) -> Self {
        self.commands.insert(name.into(), Box::new(cmd));
        self
    }

    async fn run(&mut self, user_config: &UserConfig, channels: &[String]) -> Result<(), String> {
        let connector = error_to_string(twitchchat::connector::smol::Connector::twitch())?;

        let mut runner = error_to_string(AsyncRunner::connect(connector, user_config).await)?;
        println!("{} is connecting", runner.identity.username());

        for channel in channels {
            println!("joining: {}", channel);
            if let Err(err)=runner.join(channel).await {
                eprintln!("error while joining '{}': {}", channel, err);
            }
        }

        println!("starting main loop");
        self.main_loop(&mut runner).await
    }

    async fn main_loop(&mut self, runner: &mut AsyncRunner) -> Result<(), String> {
        let mut writer = runner.writer();
        let quit = runner.quit_handle();

        loop {
            match error_to_string(runner.next_message().await)? {
                Status::Message(Commands::Privmsg(pm)) => {
                    if let Some(cmd) = Self::parse_command(pm.data()) {
                        if let Some(command) = self.commands.get_mut(cmd) {
                            println!("dispatching to: {}", cmd.escape_debug());

                            let args = Args {
                                msg: &pm,
                                writer: &mut writer,
                                quit: quit.clone(),
                            };

                            command.handle(args);
                        }
                    }
                },
                Status::Quit | Status::Eof => break,
                Status::Message(..) => continue,
            }
        }

        println!("end of main loop");
        Ok(())
    }

    fn parse_command(input: &str) -> Option<&str> {
        if input.starts_with('!') {
            input.splitn(2, ' ').next()
        } else {
            None
        }
    }
}

mod include {
    use twitchchat::UserConfig;

    fn get_env_var(key: &str) -> Result<String, String> {
        if let Ok(var) = std::env::var(key) {
            Ok(var)
        } else {
            Err(format!("please set `{}`", key))
        }
    }

    pub fn get_user_config() -> Result<UserConfig, String> {
        let name = get_env_var("TWITCH_USERNAME")?;
        let token = get_env_var("TWITCH_TOKEN")?;

        super::error_to_string(
            UserConfig::builder()
                .name(name)
                .token(token)
                .enable_all_capabilities()
                .build()
        )
    }

    pub fn channels_to_join() -> Result<Vec<String>, String> {
        Ok(
            get_env_var("TWITCH_CHANNELS")?
                .split(',')
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        )
    }
}
