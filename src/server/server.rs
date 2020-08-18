use crate::bot::Bot;
use std::env;
use tiny_http::{Server, Response};
use telegram_bot::prelude::*;

pub async fn bind_and_serve() {
  let port = env::var("PORT").expect("Missing PORT environment variable");
  let url = format!("0.0.0.0:{}", port);
  let server = Server::http(url).unwrap();
  let mut bot = Bot::new();

  // check if bot is settled
  if let Err(err) = bot.get_me().await {
    println!("Error!: {:?}", err);
    return;
  }

  for request in server.incoming_requests() {
    println!("received request: {:?}, url: {:?}, headers: {:?}",
      request.method(),
      request.url(),
      request.headers());

    let url = request.url();

    // lots of unwrap calls, horrible but is for testing purposes! :D
    if url.contains("/activate") {
      match bot.activate().await {
        Ok(_) => request.respond(Response::from_string("Bot Active!")).unwrap(),
        Err(err) => request.respond(Response::from_string(err.to_string())).unwrap()
      };

      return;
    }

    if url.contains("/send") {
      if bot.is_active {
        bot.api.spawn(bot.chat_id.unwrap().text("Hello fellow Rustaceans!"));
        request.respond(Response::from_string("Sent!")).unwrap();
        return;
      }

      request.respond(Response::from_string("Not sent! :(")).unwrap();
      return;
    }

    request.respond(Response::from_string("NOT FOUND WITHOUT STATUS CODE :D")).unwrap();
  }
}
