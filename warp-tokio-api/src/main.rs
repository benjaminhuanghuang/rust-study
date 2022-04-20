use warp::Filter;
use std::env;

#[tokio::main]
async fn main() {
  env::set_var("RUST_APP_LOG", "debug");
  pretty_env_logger::init_custom_env("RUST_APP_LOG");

  //let log = warp::log("basic");

  // GET /hello/warp => 200 OK with body "Hello, warp!"
  let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

  warp::serve(hello).run(([127, 0, 0, 1], 8964)).await;
}
  