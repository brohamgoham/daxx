use std::convert::Infallible;

use axum::{routing::get, Router};
use tokio;
mod api;
mod modelz;
use crate::api::eth;
use crate::api::route::get_txns_handler;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/v1/txns", get(txns_handler));

    axum::Server::bind(&"0.0.0.0:8494".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}

async fn txns_handler() -> Result<String, Infallible> {
    let tx = get_txns_handler().await.unwrap();
    Ok(tx)
}
