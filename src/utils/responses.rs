// use serde::{Deserialize, Serialize};
//
// #[derive(Serialize, Deserialize, Debug)]
// pub struct ApiResponse {
//     jsonrpc: String,
//     result: BlockResult,
//     id: i32,
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// struct BlockResult {
//     block_hash: String,
//     confirmed_transaction_list: Vec<Transaction>,
//     height: i64,
//     merkle_tree_root_hash: String,
//     peer_id: String,
//     prev_block_hash: String,
//     signature: String,
//     time_stamp: i64,
//     version: String,
// }
//
// // Assuming transactions can be of different types, you might define an enum for them
// // For simplicity, I'll treat them as a single type here
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(tag = "dataType", rename_all = "camelCase")]
// enum Transaction {
//     Base {
//         data: TransactionData,
//         timestamp: String,
//         txHash: String,
//         version: String,
//     },
//     // Other types of transactions can be added here
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// struct TransactionData {
//     result: TransactionResult,
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(tag = "dataType", rename_all = "camelCase")]
// struct TransactionResult {
//     covered_by_fee: String,
//     covered_by_over_issued_ICX: String,
//     issue: String,
// }