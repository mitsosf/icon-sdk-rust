use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value, Map};
use sha3::{Digest, Sha3_256};
use std::collections::BTreeMap;

#[derive(Default, Serialize, Deserialize)]
pub struct TransactionBuilder {
    icon_service_url: Option<String>,
    data: Value,
}

impl TransactionBuilder {
    pub fn new() -> Self {
        Self {
            icon_service_url: None,
            // Set default values for `data` here
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
        params.insert("value".to_string(), json!(value));

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

    pub fn serialize(&self, hashed: bool) -> String {
        let result_str = Self::value_traverse(&self.data["params"]);
        let result_string_replaced = &result_str[1..result_str.len() - 1];
        let result = format!("icx_sendTransaction.{}", result_string_replaced);

        if hashed {
            format!("{}", hex::encode(sha3::Sha3_256::digest(result.as_bytes())))
        } else {
            result
        }
    }

    fn value_traverse(value: &Value) -> String {
        match value {
            Value::Object(obj) => {
                let mut result = "{".to_string();
                let sorted: BTreeMap<_, _> = obj.iter().collect();
                for (key, val) in &sorted {
                    result.push_str(&format!("{}.", key));
                    result.push_str(&Self::value_traverse(val));
                    result.push('.');
                }
                if result.ends_with('.') {
                    result.pop();
                }
                result.push('}');
                result
            },
            Value::Array(arr) => {
                let mut result = "[".to_string();
                for val in arr {
                    result.push_str(&Self::value_traverse(val));
                    result.push('.');
                }
                if result.ends_with('.') {
                    result.pop();
                }
                result.push(']');
                result
            },
            Value::String(s) => Self::escape_string(s),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => "\\0".to_string(),
        }
    }

    fn escape_string(value: &str) -> String {
        value.replace("\\", "\\\\")
            .replace(".", "\\.")
            .replace("{", "\\{")
            .replace("}", "\\}")
            .replace("[", "\\[")
            .replace("]", "\\]")
    }

    pub async fn send(self) -> Result<Value, Error> {
        let client = Client::new();
        let url = self.icon_service_url.unwrap_or_else(|| "https://api.icon.community/api/v3".to_string());
        let data = self.data;
        println!("{:?}", data);

        let res = client.post(&url)
            .json(&data)
            .send()
            .await?;

        match res.status() {
            reqwest::StatusCode::OK => Ok(res.json().await?),
            other => panic!("HTTP Request Failed with status: {}", other),
        }
    }
}
