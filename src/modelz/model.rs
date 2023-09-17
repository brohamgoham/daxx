
use ethers::types::{Transaction, TransactionReceipt, U64, Bytes, U256};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub struct TransactionData {
    #[serde(default, rename = "blockNumber")]
    pub block_number: Option<U64>,
    pub transaction_hash: String,
    pub transaction_value: String,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<U64>,
    pub gas: u64,
    pub from: String,
    pub to: Option<String>,
    /// Input data
  //  pub input: Bytes,

    /// ECDSA recovery id
    pub v: U64,
    
    /// ECDSA signature r
    pub r: U256,
    
    /// ECDSA signature s
    pub s: U256,
    #[serde(rename = "chainId", default, skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<U256>,
}


#[derive(Debug, Serialize)]
pub struct ReceiptData {
    block_number: u64,
    transaction_hash: String,
    transaction_index: u64,
    from: String,
    to: Option<String>,
    cumulative_gas_used: u64,
    gas_used: u64,
    contract_address: Option<String>,
    // Add other fields you need here
}

