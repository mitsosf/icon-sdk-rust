use std::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use crate::transaction_builder::TransactionBuilder;
use crate::utils::transaction_types::TransactionType;
use crate::wallet::Wallet;

#[derive(Default, Serialize, Deserialize)]
pub struct IconService {
    pub(crate) icon_service_url: String,
}

impl IconService {
    pub fn new(icon_service_url: Option<String>) -> Self {
        Self {
            icon_service_url: icon_service_url
                .unwrap_or_else(|| "https://api.icon.community/api/v3".to_string())
        }
    }

    pub async fn get_last_block(&self) -> Result<Value, Box<dyn Error>> {
        let transaction = TransactionBuilder::new(self)
            .method(TransactionType::LastBlock.as_str())
            .build();

        let response: Value = transaction.send().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(response)
    }

    pub async fn get_block_by_height(&self, block_height: &str) -> Result<Value, Box<dyn Error>> {
        let transaction = TransactionBuilder::new(self)
            .method(TransactionType::BlockByHeight.as_str())
            .block_height(block_height)
            .build();

        let response: Value = transaction.send().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(response)
    }

    pub async fn get_block_by_hash(&self, block_hash: &str) -> Result<Value, Box<dyn Error>> {
        let transaction = TransactionBuilder::new(self)
            .method(TransactionType::BlockByHash.as_str())
            .block_hash(block_hash)
            .build();

        let response: Value = transaction.send().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(response)
    }

    pub async fn get_balance(&self, address: &str) -> Result<Value, Box<dyn Error>> {
        let transaction = TransactionBuilder::new(self)
            .method(TransactionType::Balance.as_str())
            .address(address)
            .build();

        let response: Value = transaction.send().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(response)
    }

    pub async fn get_transaction_result(&self, tx_hash: &str) -> Result<Value, Box<dyn Error>> {
        let transaction = TransactionBuilder::new(self)
            .method(TransactionType::TransactionResult.as_str())
            .tx_hash(tx_hash)
            .build();

        let response: Value = transaction.send().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(response)
    }

    pub async fn get_transaction_by_hash(&self, tx_hash: &str) -> Result<Value, Box<dyn Error>> {
        let transaction = TransactionBuilder::new(self)
            .method(TransactionType::TransactionByHash.as_str())
            .tx_hash(tx_hash)
            .build();

        let response: Value = transaction.send().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(response)
    }

    pub async fn call(&self, score: &str, params: Value) -> Result<Value, Box<dyn Error>> {
        let transaction = TransactionBuilder::new(self)
            .method(TransactionType::Call.as_str())
            .to(score)
            .call(params)
            .build();

        let response: Value = transaction.send().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(response)
    }

    pub async fn send_transaction(&self, wallet: Wallet, to: &str, value: &str, version: &str, nid: &str, nonce: &str, step_limit: &str) -> Result<Value, Box<dyn Error>> {
        let transaction = TransactionBuilder::new(self)
            .method(TransactionType::SendTransaction.as_str())
            .from(wallet.get_public_address().as_str())
            .to(to)
            .value(value)
            .version(version)
            .nid(nid)
            .timestamp()
            .nonce(nonce)
            .step_limit(step_limit)
            .sign(wallet.get_private_key().as_str())
            .build();

        let response: Value = transaction.send().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(response)
    }

    pub async fn send_transaction_with_message(&self, wallet: Wallet, to: &str, value: &str, version: &str, nid: &str, nonce: &str, step_limit: &str, message: &str) -> Result<Value, Box<dyn Error>> {
        let transaction = TransactionBuilder::new(self)
            .method(TransactionType::SendTransaction.as_str())
            .from(wallet.get_public_address().as_str())
            .to(to)
            .value(value)
            .version(version)
            .nid(nid)
            .timestamp()
            .nonce(nonce)
            .step_limit(step_limit)
            .message(message)
            .sign(wallet.get_private_key().as_str())
            .build();

        let response: Value = transaction.send().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(response)
    }
}
