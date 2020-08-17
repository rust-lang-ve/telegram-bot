extern crate tiny_http;

use telegram_bot::Error;

mod bot;
mod server;

#[tokio::main]
async fn main() -> Result<(), Error> {
  server::bind_and_serve().await;

  Ok(())
}
