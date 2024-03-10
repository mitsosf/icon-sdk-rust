use std::error::Error;
use serde_json::{Value};
use crate::transaction_builder::TransactionBuilder;

pub async fn get_last_block() -> Result<Value, Box<dyn Error>> {
    let transaction_builder = TransactionBuilder::new()
        .method("icx_getLastBlock");

    let response: Value = transaction_builder.send().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

    Ok(response)
}

pub async fn get_block_by_height(block_height: &str) -> Result<Value, Box<dyn Error>> {
    let transaction_builder = TransactionBuilder::new()
        .method("icx_getBlockByHeight").block_height(block_height);

    let response: Value = transaction_builder.send().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

    Ok(response)
}

pub async fn get_block_by_hash(block_hash: &str) -> Result<Value, Box<dyn Error>> {
    let transaction_builder = TransactionBuilder::new()
        .method("icx_getBlockByHash").block_hash(block_hash);

    let response: Value = transaction_builder.send().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

    Ok(response)
}

pub async fn get_balance(address: &str) -> Result<Value, Box<dyn Error>> {
    let transaction_builder = TransactionBuilder::new()
        .method("icx_getBalance").address(address);

    let response: Value = transaction_builder.send().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

    Ok(response)
}