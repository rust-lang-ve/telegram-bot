use crate::bot::Bot;
use iron::prelude::*;
use iron::status;

pub fn handle(bot: Bot, _: &mut Request) -> IronResult<Response> {
  Ok(Response::with((status::Ok, "Hello GitHub!")))
}
