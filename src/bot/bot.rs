use futures::StreamExt;
use std::env;
use telegram_bot::prelude::*;
use telegram_bot::{
  Api,
  Error,
  GetMe,
  MessageKind,
  UpdateKind,
  UpdatesStream,
  ChatId,
};

pub struct Bot {
  pub api: Api,
  pub stream: UpdatesStream,
  pub is_active: bool,
  pub chat_id: Option<ChatId>,
}

const TELEGRAM_BOT_TOKEN: &str = "TELEGRAM_BOT_TOKEN";
const ACTIVATE_COMMAND: &str = "/activate";

impl Bot {
  pub fn new() -> Self {
    let token =
      env::var(TELEGRAM_BOT_TOKEN).expect("Missing TELEGRAM_BOT_TOKEN environment variable");

    let api = Api::new(token);
    let stream = api.stream();

    Self { api, stream, is_active: false, chat_id: None }
  }

  pub fn send_to_chat(&self, message: &str) {
    if let Some(chat) = self.chat_id {
      self.api.spawn(chat.text(message));
    }
  }

  pub async fn activate(&mut self) -> Result<(), Error> {
    while let Some(update) = self.stream.next().await {
      let update = update?;

      if let UpdateKind::Message(message) = update.kind {
        match message.kind {
          MessageKind::Text { ref data, .. } if data.as_str() == ACTIVATE_COMMAND => {
            let api = self.api.clone();
            let chat = message.chat.clone();

            self.chat_id = Some(message.chat.id());
            self.is_active = true;

            tokio::spawn(async move {
              match api.send(chat.text("Hello, you have activated the RUST LANG VE BOT!".to_string())).await {
                Ok(_) => {},
                Err(err) => {
                  println!("Error!: {:?}", err);
                }
              };
            });
          },
          _ => (),
        }
      }
    }

    Ok(())
  }

  pub async fn get_me(&self) -> Result<(), Error> {
    self.api.send(GetMe).await?;

    Ok(())
  }
}
