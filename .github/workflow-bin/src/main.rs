use clap::Parser;
use pushover_rs::{send_pushover_request, PushoverSound};
use std::env;

#[derive(Parser, Debug)]
enum Commands {
    Start,
    Ok,
    Err,
}

struct Pushover {
    user_key: String,
    app_token: String,
}

impl Pushover {
    async fn send(&self, message: &str, sound: PushoverSound) -> Result<(), anyhow::Error> {
        match send_pushover_request(
            pushover_rs::MessageBuilder::new(&self.user_key, &self.app_token, message)
                .set_sound(sound)
                .build(),
        )
        .await
        {
            Ok(res) => match res.errors {
                None => Ok(()),
                Some(errs) => Err(anyhow::anyhow!("{}", errs.join("\r\n"))),
            },
            Err(err) => Err(anyhow::anyhow!("{}", err)),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let pushover = Pushover {
        user_key: env::var("PUSHOVER_USER_KEY")?,
        app_token: env::var("PUSHOVER_APP_TOKEN")?,
    };

    match Commands::parse() {
        Commands::Start => pushover.send("Workflow开始执行！", PushoverSound::BIKE),
        Commands::Ok => pushover.send("Workflow执行成功！", PushoverSound::MAGIC),
        Commands::Err => pushover.send("Workflow执行失败！", PushoverSound::FALLING),
    }
    .await
}
