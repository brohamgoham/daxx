use ethers::prelude::*;
use std::convert::Infallible;

pub async fn get_txns_handler() -> Result<String, Infallible> {
    let alchemey_api_key = std::env::var("ALCHEMEY_API_KEY").expect("ALCHEMEY_API_KEY not set");
    let url = format!(
        "https://eth-mainnet.alchemyapi.io/v2/{}",
        alchemey_api_key
    );
    // Initialize the Ethereum provider
    let provider: Provider<Http> =
        Provider::connect(&url)
            .await;


    // Get Block number
    let block_number = provider.get_block_number().await.unwrap();

    let pending_txn = provider
    .get_block_with_txs(block_number)
    .await
    .unwrap();

    let mut response_body = String::new();

    if let Some(block) = pending_txn {
        // Access the block value and its transactions
        let block_number = block.number;
        let transactions = block.transactions;

        // Process the transactions or access their properties
        for transaction in transactions {
            // Access transaction properties (e.g., transaction.hash, transaction.from, transaction.to)
            let transaction_hash = format!("Transaction hash 💫 : {:#?}", transaction.hash);
            let transaction_value = format!("Transaction value💰 : {:#?}", transaction.value);
            let transaction_type =
                format!("Transaction Type 🚨: {:#?}", transaction.transaction_type);
            let gas = format!("Gas⛽️: {:#?}", transaction.gas);
            let from = format!("From 😈: {:#?}", transaction.from);
            let to = format!("To 🌙: {:#?}", transaction.to);

            let transaction_info = format!(
                "{}\n{}\n{}\n{}\n{}\n{}\n",
                transaction_hash, transaction_value, transaction_type, gas, from, to,
            );

            response_body.push_str(&transaction_info);
        }

        // Use the block number and transactions as needed
        response_body.push_str(&format!("Block number: {:#?}", block_number));
    } else {
        response_body = "No pending transactions found.".to_string();
    }

    Ok(response_body)
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_get_txns_handler() {
        let result = get_txns_handler().await;

        dbg!("Result: {:#?}", &result);
        let result2 = get_txns_handler().await;

        dbg!("Result2: {:#?}", &result2);
    }
}
