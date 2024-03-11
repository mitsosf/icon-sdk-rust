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

Fully or partially supports all Iconservice functions, IRC-2 tokens, and IISS calls.

Installation
--------

To use the SDK in your Rust project, add the following to your `Cargo.toml`:


```toml
[dependencies]
icon-sdk = "1.0.1"
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
    let last_block = icon_service::get_last_block().await;
    println!("{:?}", last_block);

    // Example: get block by height
    let block_by_height = icon_service::get_block_by_height("0x3").await;
    println!("{:?}", block_by_height);

    // Example: get block by hash
    let block_by_hash = icon_service::get_block_by_hash("0x123986e1c834632f6e65915c249d81cd01453ec915e3370d364d6df7be5e6c03").await;
    println!("{:?}", block_by_hash);
}
```

### Send ICX 
```rust
use icon_sdk::{icon_service, wallet::Wallet};

#[tokio::main]
async fn main() {
    let wallet = Wallet::new(None); //Or load a wallet from a private key

    let from = wallet.get_public_address(); // Sender's address
    let to = "hx9ab3078e72c8d9017194d17b34b1a47b661945ca";
    let value = "100"; // Amount to send in ICX or hex encoded value for tokens
    let version = "0x3"; 
    let nid = "0x3"; 
    let nonce = "0x1234";
    let step_limit = "0x186a0"; 

    // Send the transaction
    match icon_service::send_transaction(wallet, from, to, value, version, nid, nonce, step_limit).await {
        Ok(response) => println!("Transaction sent successfully: {:?}", response),
        Err(err) => eprintln!("Error sending transaction: {}", err),
    }
}
```

Coming soon
--------
- More examples
- More RPC calls implemented