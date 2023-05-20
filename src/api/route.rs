use ethers::prelude::*;
use std::convert::Infallible;
use ethers::types::H256;
use std::str::FromStr;

pub async fn get_txns_handler() -> Result<String, Infallible> {

    // Initialize the Ethereum provider
    let provider: Provider<Http> =
        Provider::connect("https://eth-mainnet.alchemyapi.io/v2/o_bo9q2LMtGvYqr7jsyYSpUrE_azdh9x")
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
                "{transaction_hash}\n{transaction_value}\n{transaction_type}\n{gas}\n{from}\n{to}\n",
            );

            response_body.push_str(&transaction_info);
        }

        // Use the block number and transactions as needed
        response_body.push_str(&format!("Block number: {block_number:#?}"));
    } else {
        response_body = "No pending transactions found.".to_string();
    }

    Ok(response_body)
}

pub async fn get_receipts(txn_hash: DaxxTxnHash) -> Result<String, Infallible> {
    // Initialize the Ethereum provider
    let provider: Provider<Http> =
        Provider::connect("https://eth-mainnet.alchemyapi.io/v2/o_bo9q2LMtGvYqr7jsyYSpUrE_azdh9x")
            .await;

    let txn_receipt = provider
            .get_transaction_receipt(txn_hash)
            .await
            .unwrap();

    let mut response_body = String::new();

    if let Some(receipt) = txn_receipt {
        // Access the block value and its transactions
        let block_number = receipt.block_number;
        let transaction_hash = receipt.transaction_hash;
        let transaction_index = receipt.transaction_index;
        let from = receipt.from;
        let to = receipt.to;
        let cumulative_gas_used = receipt.cumulative_gas_used;
        let gas_used = receipt.gas_used;
        let contract_address = receipt.contract_address;
        let logs = receipt.logs;
        let logs_bloom = receipt.logs_bloom;
        let status = receipt.status;

        let receipt_info = format!(
            "Block number: {block_number:#?}\nTransaction hash: {transaction_hash:#?}\nTransaction index: {transaction_index:#?}\nFrom: {from:#?}\nTo: {to:#?}\nCumulative gas used: {cumulative_gas_used:#?}\nGas used: {gas_used:#?}\nContract address: {contract_address:#?}\nLogs: {logs:#?}\nLogs bloom: {logs_bloom:#?}\nStatus: {status:#?}\n"
        );

        response_body.push_str(&receipt_info);
    } else {
        response_body = "No pending transactions found.".to_string();
    }

    Ok(response_body)


}

pub struct DaxxTxnHash {
    txn_hash: TxHash,
}

impl DaxxTxnHash {
    pub fn new(txn_hash: TxHash) -> Self {
        Self { txn_hash }
    }

}

impl From<DaxxTxnHash> for H256 {
    fn from(daxx_txn_hash: DaxxTxnHash) -> Self {
        Self::from_slice(&daxx_txn_hash.txn_hash.0)
    }
}
//convert daxx txn hash to eth txn hash
impl From<DaxxTxnHash> for String {
    fn from(daxx_txn_hash: DaxxTxnHash) -> Self {
        format!("{:#?}", daxx_txn_hash.txn_hash)
    }
}

impl FromStr for DaxxTxnHash {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let txn_hash = s
            .strip_prefix("0x")
            .ok_or(())?
            .parse::<TxHash>()
            .map_err(|_| ())?;

        Ok(Self::new(txn_hash))
    }
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

    #[tokio::test]
    async fn test_get_tx_receipt() {
        let txn_hash = "0x390be29230671dcc9628689761da0e7bf710fe883e5c29d83f00e092a6cd24d6"
            .parse::<TxHash>()
            .expect("failed to parse txn hash");
        let daxx_txn_hash = DaxxTxnHash::new(txn_hash);
        let res = get_receipts(daxx_txn_hash).await;
        dbg!("Result: {:#?}", &res);
    }
}
