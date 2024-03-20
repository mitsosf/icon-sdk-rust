<p align="center">
  <img 
    src="https://iconation.team/images/very_small.png" 
    width="120px"
    alt="ICONation logo">
</p>

<h1 align="center">ICON SDK for Rust</h1>

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This is an SDK to communicate with the ICON blockchain, built for Rust.

Disclaimer: I cannot guarantee optimal performance of this software. It is provided as is and without any assurances. Use it at your own risk.

Features
--------

- Wallet management
- Read data from the blockchain
- Send ICX transactions
- Full IRC2 token support
- Perform SCORE calls
- Transaction builder

Installation
--------

To use the SDK in your Rust project, add the following to your `Cargo.toml`:


```toml
[dependencies]
icon-sdk = "1.2.0"
```

Testing
--------
To run tests, ensure you have Rust installed and run:
```shell
cargo test
```

Usage
--------
### Get block information
```rust
use icon_sdk::icon_service;

async fn main() {
    // Example: get last block
    let icon_service = icon_service::IconService::new(None);
    let last_block = icon_service.get_last_block().await;
    println!("{:?}", last_block);

    // Example: get block by height
    let block_by_height = icon_service.get_block_by_height("0x3").await;
    println!("{:?}", block_by_height);

    // Example: get block by hash
    let block_by_hash = icon_service.get_block_by_hash("0x123986e1c834632f6e65915c249d81cd01453ec915e3370d364d6df7be5e6c03").await;
    println!("{:?}", block_by_hash);
    
    // Example: get transaction result
    let transaction_result = icon_service.get_transaction_result("0x123986e1c834632f6e65915c249d81cd01453ec915e3370d364d6df7be5e6c03").await;
    println!("{:?}", transaction_result);
    
    // Example: get transaction by hash
    let transaction_by_hash = icon_service.get_transaction_by_hash("0x123986e1c834632f6e65915c249d81cd01453ec915e3370d364d6df7be5e6c03").await;
    println!("{:?}", transaction_by_hash);
    
    // Example: SCORE call
    let score_call = icon_service.call(
        "cx9ab3078e72c8d9017194d17b34b1a47b661945ca",
        json!({
            "method": "balanceOf",
            "params": {
                "_owner": "hx70e8eeb5d23ab18a828ec95f769db6d953e5f0fd"
            }
        })).await;
    println!("{:?}", score_call);
}
```

### Send ICX 
```rust
use icon_sdk::{icon_service, wallet::Wallet};

#[tokio::main]
async fn main() {
    let wallet = Wallet::new(None); //Or load a wallet from a private key

    let to = "hx9ab3078e72c8d9017194d17b34b1a47b661945ca";
    let value = "100"; // Amount to send in ICX or hex encoded value for tokens
    let version = "0x3"; 
    let nid = "0x3"; 
    let nonce = "0x1234";
    let step_limit = "0x186a0";
    let message = "Hello, ICON!";

    let icon_service = icon_service::IconService::new(None);
    // Send the transaction
    match icon_service.send_transaction(wallet, to, value, version, nid, nonce, step_limit).await {
        Ok(response) => println!("Transaction sent successfully: {:?}", response),
        Err(err) => eprintln!("Error sending transaction: {}", err),
    }
    
    // Send the transaction with a message
    match icon_service.send_transaction_with_message(wallet, to, value, version, nid, nonce, step_limit, message).await {
        Ok(response) => println!("Transaction sent successfully: {:?}", response),
        Err(err) => eprintln!("Error sending transaction: {}", err),
    }
}
```

### Use the testnet
```rust
// Lisbon testnet, make sure to also change the nid when needed
let icon_service = icon_service::IconService::new(Some("https://lisbon.net.solidwallet.io/api/v3".to_string()));
```

### Use the transaction builder
See `icon_service.rs` to see how to use the transaction builder.

