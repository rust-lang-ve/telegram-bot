use crate::bot::Bot;
use std::env;
use tiny_http::{Server, Response};

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

  match bot.activate().await {
    Ok(_) => {
      for request in server.incoming_requests() {
        println!("received request: {:?}, url: {:?}, headers: {:?}",
          request.method(),
          request.url(),
          request.headers());

        if bot.is_active {
          bot.send_to_chat("Hello fellow Rustaceans!");
        }

        let response = Response::from_string("Hello fellow Rustaceans!");
        if let Err(err) = request.respond(response) {
          println!("Error!: {:?}", err);
        }
      }
    },
    Err(err) => {
      println!("Error!: {:?}", err);
    }
  }
}
