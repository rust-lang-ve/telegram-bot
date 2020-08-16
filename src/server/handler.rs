use crate::bot::Bot;
use crate::server::hooks::github;
use iron::prelude::*;
use iron::status;

pub fn handler(req: &mut Request) -> IronResult<Response> {
  let bot = Bot::new();
  let req_host = req.url.host().to_owned();

  match req_host.to_string().as_str() {
    "github.com" => {
      github::handle(bot, req)
    },
    _ => {
      Ok(Response::with((status::NotFound, "Not Found")))
    }
  }
}
