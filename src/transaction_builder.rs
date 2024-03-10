use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value, Map};

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
