use twitchchat::{
    messages::{Commands, Privmsg},
    runner::{AsyncRunner, NotifyHandle, Status},
    UserConfig,
};
use std::collections::HashMap;

pub const PREFIX: char = '!';

pub mod dice;
pub mod include;

pub(crate) fn error_to_string<T>(res: Result<T, impl std::error::Error>) -> Result<T, String> {
    res.map_err(|e| e.to_string())
}

pub struct Args<'a, 'b: 'a> {
    pub msg: &'a Privmsg<'b>,
    pub writer: &'a mut twitchchat::Writer,
    pub quit: NotifyHandle,
    pub params: Vec<&'a str>,
}

pub trait Command: Send + Sync {
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
pub struct Bot {
    commands: HashMap<String, Box<dyn Command>>,
}

impl Bot {
    pub fn with_command(mut self, name: impl Into<String>, cmd: impl Command + 'static) -> Self {
        self.commands.insert(name.into(), Box::new(cmd));
        self
    }

    pub async fn run(&mut self, user_config: &UserConfig, channels: &[String]) -> Result<(), String> {
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
                    if let Some(cmd_vec) = Self::parse_command(pm.data()) {
                        let cmd = cmd_vec[0];
                        let params = cmd_vec.iter().skip(1).map(|s| *s).collect();
                        if let Some(command) = self.commands.get_mut(cmd) {
                            println!("dispatching to: {}", cmd.escape_debug());

                            let args = Args {
                                msg: &pm,
                                writer: &mut writer,
                                quit: quit.clone(),
                                params,
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

    fn parse_command(input: &str) -> Option<Vec<&str>> {
        if input.starts_with(PREFIX) {
            Some(input.split(' ').filter(|&s| s.len() != 0).collect())
        } else {
            None
        }
    }
}
