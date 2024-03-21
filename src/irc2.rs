use std::error::Error;
use std::str::FromStr;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::icon_service::IconService;
use crate::transaction_builder::TransactionBuilder;
use crate::utils::helpers::icx_to_hex;
use crate::utils::transaction_types::TransactionType;
use crate::wallet::Wallet;

#[derive(Default, Serialize, Deserialize)]
pub struct IRC2 {
    icon_service: IconService,
    contract_address: String,
}

impl IRC2 {
    pub fn new(icon_service: IconService, contract_address: String) -> Self {
        Self {
            icon_service,
            contract_address,
        }
    }

    pub async fn name(&self) -> Result<Value, Box<dyn Error>> {
        let transaction = TransactionBuilder::new(&self.icon_service)
            .method(TransactionType::Call.as_str())
            .to(&self.contract_address)
            .call(
                json!({
                    "method": "name",
                })
            )
            .build();

        let response: Value = transaction.send().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(response)
    }

    pub async fn symbol(&self) -> Result<Value, Box<dyn Error>> {
        let transaction = TransactionBuilder::new(&self.icon_service)
            .method(TransactionType::Call.as_str())
            .to(&self.contract_address)
            .call(
                json!({
                    "method": "symbol",
                })
            )
            .build();

        let response: Value = transaction.send().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(response)
    }

    pub async fn decimals(&self) -> Result<Value, Box<dyn Error>> {
        let transaction = TransactionBuilder::new(&self.icon_service)
            .method(TransactionType::Call.as_str())
            .to(&self.contract_address)
            .call(
                json!({
                    "method": "decimals",
                })
            )
            .build();

        let response: Value = transaction.send().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(response)
    }

    pub async fn total_supply(&self) -> Result<Value, Box<dyn Error>> {
        let transaction = TransactionBuilder::new(&self.icon_service)
            .method(TransactionType::Call.as_str())
            .to(&self.contract_address)
            .call(
                json!({
                    "method": "totalSupply",
                })
            )
            .build();

        let response: Value = transaction.send().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(response)
    }

    pub async fn balance_of(&self, account: String) -> Result<Value, Box<dyn Error>> {
        let transaction = TransactionBuilder::new(&self.icon_service)
            .method(TransactionType::Call.as_str())
            .to(&self.contract_address)
            .call(
                json!({
                    "method": "balanceOf",
                    "params": {
                        "_owner": account,
                    }
                })
            )
            .build();

        let response: Value = transaction.send().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(response)
    }

    pub async fn transfer(&self, wallet: Wallet, to: &str, value: &str, version: &str, nid: &str, nonce: &str, step_limit: &str) -> Result<Value, Box<dyn Error>> {
        let mut parsed_value = value.to_string();

        if !parsed_value.starts_with("0x") {
            match icx_to_hex(Decimal::from_str(value).expect("Invalid value")) {
                Some(v) => {
                    parsed_value = v;
                }
                None => panic!("Failed to convert value to hex"),
            }
        }

        let transaction = TransactionBuilder::new(&self.icon_service)
            .method(TransactionType::SendTransaction.as_str())
            .from(wallet.get_public_address().as_str())
            .to(&self.contract_address)
            .version(version)
            .nid(nid)
            .timestamp()
            .nonce(nonce)
            .step_limit(step_limit)
            .call(
                json!({
                    "method": "transfer",
                    "params": {
                        "_to": to,
                        "_value": parsed_value,
                    }
                })
            )
            .sign(wallet.get_private_key().as_str())
            .build();

        let response: Value = transaction.send().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(response)
    }

}
