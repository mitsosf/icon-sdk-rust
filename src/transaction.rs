use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use thiserror::Error;
use crate::icon_service::IconService;

#[derive(Default, Serialize, Deserialize)]
pub struct Transaction {
    icon_service_url: String,
    pub(crate) data: Value,
}

#[derive(Error, Debug)]
pub enum MyError {
    #[error("request failed")]
    Request(#[from] reqwest::Error),

    #[error("unexpected response: {0}")]
    UnexpectedResponse(String),
}

impl Transaction {
    pub fn new(icon_service: &IconService) -> Self {
        Self {
            icon_service_url: format!("{}", icon_service.icon_service_url),
            data: json!({
                "jsonrpc": "2.0",
                "id": 1234
            }),
        }
    }

    pub async fn send(self) -> Result<Value, MyError> {
        let client = Client::new();
        let url = self.icon_service_url;
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
