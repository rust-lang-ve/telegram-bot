use crate::server::handler::handler;
use std::env;
use iron::prelude::*;

pub fn bind_and_serve() {
  let port = env::var("PORT")
    .expect("Missing PORT environment variable");
  let chain = Chain::new(handler);
  let url = format!("0.0.0.0:{}", port);

  Iron::new(chain).http(&url[..]).unwrap();
  println!("Bound on {:?}", url);
}
