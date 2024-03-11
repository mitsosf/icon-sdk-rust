use serde_json::Value;
use sha3::{Digest, Sha3_256};
use std::collections::BTreeMap;
use hex::encode;

pub struct Serializer;

impl Serializer {
    pub fn serialize_transaction(data: &Value, hashed: bool) -> String {
        let result_str = Self::value_traverse(data);
        let result_string_replaced = &result_str[1..result_str.len() - 1];
        let result = format!("icx_sendTransaction.{}", result_string_replaced);

        if hashed {
            format!("{}", encode(Sha3_256::digest(result.as_bytes())))
        } else {
            result
        }
    }

    fn value_traverse(value: &Value) -> String {
        match value {
            Value::Object(obj) => {
                let mut result = "{".to_string();
                let sorted: BTreeMap<_, _> = obj.iter().collect();
                for (key, val) in sorted {
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
}
