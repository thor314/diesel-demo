#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

mod demo;
mod error;
mod schema;
#[cfg(test)] mod tests;
mod utils;

use axum::{
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
  Router,
};
use error::MyError;
use tracing::info;

async fn hello_world() -> &'static str { "Hello, world!" }

async fn error_handler() -> impl IntoResponse {
  (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
}

#[shuttle_runtime::main]
async fn main(
  #[shuttle_secrets::Secrets] secret_store: shuttle_secrets::SecretStore,
) -> shuttle_axum::ShuttleAxum {
  utils::setup(&secret_store).unwrap();

  info!("hello thor");
  let mut conn = demo::establish_connection(&secret_store).await;

  demo::create_posts::create_post(&mut conn, "title", "body").await;
  demo::create_posts::create_post(&mut conn, "title2", "body2").await;
  demo::publish_post::publish_post(&mut conn, 1).await;
  demo::publish_post::publish_post(&mut conn, 2).await;
  demo::show_posts::show_posts(&mut conn).await;

  let router = Router::new()
    .route("/", get(hello_world))
    .route("/-1/error", get(error_handler))
    .route("/-1/health", get(|| async { StatusCode::OK }));

  Ok(router.into())
}
