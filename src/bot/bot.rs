use futures::StreamExt;
use std::env;
use telegram_bot::{
  Api,
  CanReplySendMessage,
  Error,
  MessageKind,
  UpdateKind,
  UpdatesStream,
};

pub struct Bot {
  pub api: Api,
  pub stream: UpdatesStream,
}

const TELEGRAM_BOT_TOKEN: &str = "TELEGRAM_BOT_TOKEN";

impl Bot {
  pub fn new() -> Self {
    let token =
      env::var(TELEGRAM_BOT_TOKEN).expect("Missing TELEGRAM_BOT_TOKEN environment variable");

    let api = Api::new(token);
    let stream = api.stream();

    Self { api, stream }
  }

  pub async fn listen(&mut self) -> Result<(), Error> {
    while let Some(update) = self.stream.next().await {
      // check if the received update contains a message
      let update = update?;

      if let UpdateKind::Message(message) = update.kind {
        if let MessageKind::Text { ref data, .. } = message.kind {
          // Answer message with "Hi".
          self
            .api
            .send(message.text_reply(format!(
              "Hi, {}! You just wrote '{}'",
              &message.from.first_name, data
            )))
            .await?;
        }
      }
    }

    Ok(())
  }
}
