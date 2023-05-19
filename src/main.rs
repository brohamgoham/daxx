use std::convert::Infallible;

use axum::{routing::get, Router};
use tokio;
mod api;
mod modelz;
use crate::api::eth;
use crate::api::route::*;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/v1/txns", get(txns_handler))
        .route("/v1/txns/:txn_hash", get(receipt_handler));

    axum::Server::bind(&"0.0.0.0:8494".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}

async fn txns_handler() -> Result<String, Infallible> {
    let tx = get_txns_handler().await.unwrap();
    Ok(tx)
}

async fn receipt_handler(tx: DaxxTxnHash) -> Result<String, Infallible> {
    let tx = get_receipts(tx).await.unwrap();
    Ok(tx)
}