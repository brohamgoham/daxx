use anyhow::Result;
use ethers::prelude::*;
use ethers::types::U256;
use reqwest::Client;

pub async fn get_block_number_and_pending_txn() -> String {
    // Initialize the Ethereum provider
    let provider: Provider<Http> =
        Provider::connect("https://eth-mainnet.g.alchemy.com/v2/o_bo9q2LMtGvYqr7jsyYSpUrE_azdh9x")
            .await;

    // Get all transactions for the specified address
    let block_number = provider.get_block_number().await.unwrap();

    let pending_txn = provider.get_block_with_txs(block_number).await.unwrap();

    if let Some(block) = pending_txn {
        // Access the block value and its transactions
        let block_number = block.number;
        let transactions = block.transactions;

        // Process the transactions or access their properties
        for transaction in transactions {
            // Access transaction properties (e.g., transaction.hash, transaction.from, transaction.to)
            println!("Transaction hash 💫 : {:#?}", transaction.hash);
            println!("Transaction value💰 : {:#?}", transaction.value);
            println!("Transaction Type 🚨: {:#?}", transaction.transaction_type);
            println!("Gas⛽️: {:#?}", transaction.gas);
            println!("From 😈: {:#?}", transaction.from);
            println!("To 🌙: {:#?}", transaction.to);
        }

        // Use the block number and transactions as needed
        println!("Block number: {:#?}", block_number);
    } else {
        println!("No pending transactions found.");
    }
    format!("Block number: {}", block_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_block_number_and_pending_txn() -> Result<(), anyhow::Error> {
        let result = get_block_number_and_pending_txn().await;

        dbg!("Result: {:#?}", &result);
        let result2 = get_block_number_and_pending_txn().await;
        dbg!("Result2: {:#?}", &result2);

        Ok(())
    }
}
