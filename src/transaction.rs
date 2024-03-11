use std::str::FromStr;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value, Map};
use secp256k1::{Message, Secp256k1, SecretKey};
use hex::{encode, decode};
use crate::utils::serializer::Serializer;
use chrono::Utc;
use base64::{Engine as _, engine::{general_purpose as base64_encoder}};
use hex::FromHex;
use rust_decimal::Decimal;

use thiserror::Error;
use crate::utils::helpers::icx_to_hex;

#[derive(Default, Serialize, Deserialize)]
pub struct Transaction {
    icon_service_url: Option<String>,
    data: Value,
}

#[derive(Error, Debug)]
pub enum MyError {
    #[error("request failed")]
    Request(#[from] reqwest::Error),

    #[error("unexpected response: {0}")]
    UnexpectedResponse(String),
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            icon_service_url: None,
            data: json!({
                "jsonrpc": "2.0",
                "id": 1234
            }),
        }
    }

    pub fn icon_service_url(mut self, url: &str) -> Self {
        self.icon_service_url = Some(url.to_string());
        self
    }

    pub fn method(mut self, method: &str) -> Self {
        // Use as_object_mut() to get a mutable reference to the data object
        if let Some(obj) = self.data.as_object_mut() {
            // Insert or modify the "method" field
            obj.insert("method".to_string(), json!(method));
        }
        self
    }

    pub fn set_params(mut self, params: &Map<String, Value>) -> Self {
        // Ensure `data` has a "params" object; create it if not
        let data_obj = self.data.as_object_mut().expect("data is not an object");
        let params_obj = data_obj.entry("params").or_insert_with(|| json!({})).as_object_mut().unwrap();

        // Insert or update the given parameters
        for (key, value) in params {
            params_obj.insert(key.clone(), value.clone());
        }

        self
    }

    pub fn block_height(self, block_height: &str) -> Self {
        let mut params = Map::new();
        params.insert("height".to_string(), json!(block_height));

        self.set_params(&params)
    }

    pub fn block_hash(self, block_hash: &str) -> Self {
        let mut params = Map::new();
        params.insert("hash".to_string(), json!(block_hash));

        self.set_params(&params)
    }

    pub fn address(self, address: &str) -> Self {
        let mut params = Map::new();
        params.insert("address".to_string(), json!(address));

        self.set_params(&params)
    }

    pub fn from(self, from: &str) -> Self {
        let mut params = Map::new();
        params.insert("from".to_string(), json!(from));

        self.set_params(&params)
    }

    pub fn to(self, to: &str) -> Self {
        let mut params = Map::new();
        params.insert("to".to_string(), json!(to));

        self.set_params(&params)
    }

    pub fn value(self, value: &str) -> Self {
        let mut params = Map::new();
        let mut parsed_value = value.to_string();

        if !parsed_value.starts_with("0x") {
            match icx_to_hex(Decimal::from_str(value).expect("Invalid value")) {
                Some(v) => {
                    parsed_value = v;
                }
                None => panic!("Failed to convert value to hex"),
            }
        }

        params.insert("value".to_string(), json!(parsed_value));


        self.set_params(&params)
    }

    pub fn version(self, version: &str) -> Self {
        let mut params = Map::new();
        params.insert("version".to_string(), json!(version));

        self.set_params(&params)
    }

    pub fn nid(self, nid: &str) -> Self {
        let mut params = Map::new();
        params.insert("nid".to_string(), json!(nid));

        self.set_params(&params)
    }

    pub fn nonce(self, nonce: &str) -> Self {
        let mut params = Map::new();
        params.insert("nonce".to_string(), json!(nonce));

        self.set_params(&params)
    }

    pub fn step_limit(self, step_limit: &str) -> Self {
        let mut params = Map::new();
        params.insert("stepLimit".to_string(), json!(step_limit));

        self.set_params(&params)
    }

    pub fn timestamp(self) -> Self {
        let now = Utc::now();
        let timestamp_in_micros = now.timestamp_micros();
        let hex_timestamp = format!("0x{:x}", timestamp_in_micros);

        let mut params = Map::new();
        params.insert("timestamp".to_string(), json!(hex_timestamp.to_string()));

        self.set_params(&params)
    }

    pub fn sign(self, private_key: &str) -> Self {
        let serialized_transaction = Serializer::serialize_transaction(&self.data["params"], true);
        let serialized_transaction_bytes = Vec::from_hex(serialized_transaction).expect("Invalid hex string");

        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&decode(private_key).expect("Invalid private key")).expect("Failed to create secret key");

        let message = Message::from_digest_slice(serialized_transaction_bytes.as_slice()).expect("Failed to create message");
        let sig = secp.sign_ecdsa_recoverable(&message, &secret_key);

        let (rec_id, sig_bytes) = sig.serialize_compact();
        // Concatenate r, s, and recovery ID
        let signature = format!("{}{:02x}", encode(sig_bytes), rec_id.to_i32());

        let signature_bytes = decode(signature).expect("Failed to decode hex");
        let transaction_signature = base64_encoder::STANDARD.encode(signature_bytes);

        let mut params = Map::new();
        params.insert("signature".to_string(), json!(transaction_signature));

        self.set_params(&params)
    }

    pub fn get_signature(self) -> Value{
        self.data["params"]["signature"].clone()
    }



    pub async fn send(self) -> Result<Value, MyError> {
        let client = Client::new();
        let url = self.icon_service_url.unwrap_or_else(|| "https://api.icon.community/api/v3".to_string());
        let data = self.data;

        let res = client.post(&url)
            .json(&data)
            .send()
            .await?;

        match res.status() {
            StatusCode::OK => Ok(res.json().await?),
            _ => {
                let error_message = res.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                Err(MyError::UnexpectedResponse(error_message))
            },
        }
    }
}
