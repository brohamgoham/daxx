use tokio;

mod api;
mod modelz;
use crate::api::eth;

#[tokio::main]
async fn main() {
    api::eth::get_block_number_and_pending_txn();
}
