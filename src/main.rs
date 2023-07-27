// #![allow(unused)]

use std::net::SocketAddr;

use axum::Router;
use axum::response::Html;
use axum::routing::get;

#[tokio::main]
async fn main() {
  // Routing 
  let routes = Router::new().route(
    "/hello",
    get(|| async { Html("Hello<strong>World!!</strong>") }),
  );

  // タプル で、ip, portを引数に渡す
  let addr = SocketAddr::from(([127, 0, 0, 3], 80));
  println!("->> LISTENING on {addr}\n");

  // bind
  axum::Server::bind(&addr)
    .serve(routes.into_make_service())
    .await
    .unwrap();
}