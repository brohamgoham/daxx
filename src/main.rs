use std::convert::Infallible;

use axum::extract::Path;
use axum::{routing::get, Router};
use tokio;

mod api;
mod modelz;

use crate::api::route::*;

extern crate pretty_env_logger;
#[macro_use] extern crate log;


#[tokio::main]
async fn main() {
    pretty_env_logger::init_custom_env("DAXX_LOG_LEVEL");
    info!(" 🌙 🌙 🌙 🌙 🌙 🌙 🌙 🌙 🌙 🌙 🌙 Starting Daxx API server...  🌙 🌙 🌙 🌙 🌙 🌙 🌙 🌙 🌙 🌙 🌙 🌙");
    let app = Router::new()
        .route("/v1/txns", get(txns_handler))
        .route("/v1/txns/:txn_hash", get(receipt_handler));

    axum::Server::bind(&"0.0.0.0:8494".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}

async fn txns_handler() -> Result<String, Infallible> {
    info!("😈 Getting Your Transactions 🚨...");
    let tx = get_txns_handler().await.unwrap();
    Ok(tx)
}

async fn receipt_handler(Path(txn_hash): Path<String>) -> Result<String, Infallible> {
    info!("😈 Getting  Receipt for Tx Hash: {:#?} 🚨...", txn_hash);
    let tx = txn_hash
        .parse::<DaxxTxnHash>()
        .expect("failed to parse txn hash");
    let tx = get_receipts(tx).await.unwrap();
    Ok(tx)
}