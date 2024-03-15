use std::str::FromStr;
use rust_decimal::Decimal;
use serde_json::json;
use icon_sdk::icon_service;
use icon_sdk::utils::helpers;
use icon_sdk::wallet::Wallet;

#[tokio::test]
async fn test_get_last_block() -> Result<(), ()> {
    let icon_service = icon_service::IconService::new(None);
    let res = icon_service.get_last_block().await;
    match res {
        Ok(response) => {
            assert_eq!(response["jsonrpc"], "2.0");
            assert!(!response.as_object().unwrap().contains_key("error"));
            println!("{:?}", response);
        },
        Err(e) => panic!("Error: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
async fn test_get_block_by_height() -> Result<(), ()> {
    let icon_service = icon_service::IconService::new(None);
    let res = icon_service.get_block_by_height("0x0").await;
    match res {
        Ok(response) => {
            assert_eq!(response["jsonrpc"], "2.0");
            assert!(!response.as_object().unwrap().contains_key("error"));
            println!("{:?}", response);
        },
        Err(e) => panic!("Error: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
async fn test_get_block_by_hash() -> Result<(), ()> {
    let icon_service = icon_service::IconService::new(None);
    let res = icon_service.get_block_by_hash("0xcf43b3fd45981431a0e64f79d07bfcf703e064b73b802c5f32834eec72142190").await;
    match res {
        Ok(response) => {
            assert_eq!(response["jsonrpc"], "2.0");
            assert!(!response.as_object().unwrap().contains_key("error"));
            println!("{:?}", response);
        },
        Err(e) => panic!("Error: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
async fn test_get_balance() -> Result<(), ()> {
    let icon_service = icon_service::IconService::new(None);
    let res = icon_service.get_balance("hxd5ace539bf910635c2fa0e9c185d2d3c8d52c4cc").await;
    match res {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response["jsonrpc"], "2.0");
            assert!(!response.as_object().unwrap().contains_key("error"));
        },
        Err(e) => panic!("Error: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
async fn test_get_transaction_result() -> Result<(), ()> {
    let icon_service = icon_service::IconService::new(None);
    let res = icon_service.get_transaction_result("0x1b6133792cee1ab2e54ae68faf9f49daf81c7e46d68b1ca281acc718602c77dd").await;
    match res {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response["jsonrpc"], "2.0");
            assert!(!response.as_object().unwrap().contains_key("error"));
        },
        Err(e) => panic!("Error: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
async fn test_get_transaction_by_hash() -> Result<(), ()> {
    let icon_service = icon_service::IconService::new(None);
    let res = icon_service.get_transaction_by_hash("0x1b6133792cee1ab2e54ae68faf9f49daf81c7e46d68b1ca281acc718602c77dd").await;
    match res {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response["jsonrpc"], "2.0");
            assert!(!response.as_object().unwrap().contains_key("error"));
        },
        Err(e) => panic!("Error: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
async fn test_call() -> Result<(), ()> {
    let icon_service = icon_service::IconService::new(None);
    let res = icon_service.call(
        "cx9ab3078e72c8d9017194d17b34b1a47b661945ca",
        json!({
            "method": "balanceOf",
            "params": {
                "_owner": "hx70e8eeb5d23ab18a828ec95f769db6d953e5f0fd"
            }
        })).await;
    match res {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response["jsonrpc"], "2.0");
            assert!(!response.as_object().unwrap().contains_key("error"));
        },
        Err(e) => panic!("Error: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
async fn test_hex_to_icx() -> Result<(), ()> {
    let res = helpers::hex_to_icx("0x63b5429420c741b16a10f");
    match res {
        Some(response) => {
            assert_eq!(response.to_string(), "7533727.039631672546337039");
        },
        None => panic!("Error"),
    }

    Ok(())
}

#[tokio::test]
async fn test_icx_to_hex() -> Result<(), ()> {
    let res = helpers::icx_to_hex(Decimal::from_str("7533727.039631672546337039").unwrap());
    match res {
        Some(response) => {
            assert_eq!(response, "0x63b5429420c741b16a10f");
            println!("{:?}", response);
        },
        None => panic!("Error"),
    }

    Ok(())
}

#[tokio::test]
async fn test_send_transaction() -> Result<(), ()> {
    let wallet = Wallet::new(Some("f4ade1ff528c9e0bf10d35909e3486ef6ce88df8a183fc1cc2c65bfa9a53d3fd".to_string()));

    let icon_service = icon_service::IconService::new(Some("https://lisbon.net.solidwallet.io/api/v3".to_string()));
    let res = icon_service.send_transaction(
        wallet,
        "hxf8689d6c4c8f333651469fdea2ac59a18f6c2421",
        "1.31231232",
        "0x3",
        "0x2",
        "0x1",
        "0x186a0"
    ).await;

    match res {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response["jsonrpc"], "2.0");
            assert!(!response.as_object().unwrap().contains_key("error"));
        },
        Err(e) => panic!("Error: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
async fn test_send_transaction_with_message() -> Result<(), ()> {
    let wallet = Wallet::new(Some("f4ade1ff528c9e0bf10d35909e3486ef6ce88df8a183fc1cc2c65bfa9a53d3fd".to_string()));

    let icon_service = icon_service::IconService::new(Some("https://lisbon.net.solidwallet.io/api/v3".to_string()));
    let res = icon_service.send_transaction_with_message(
        wallet,
        "hxf8689d6c4c8f333651469fdea2ac59a18f6c2421",
        "1.31231232",
        "0x3",
        "0x2",
        "0x1",
        "0x186a00",
        "Test message"
    ).await;

    match res {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response["jsonrpc"], "2.0");
            assert!(!response.as_object().unwrap().contains_key("error"));
        },
        Err(e) => panic!("Error: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
async fn test_wallet() -> Result<(), ()> {
    let wallet = Wallet::new(Some("f4ade1ff528c9e0bf10d35909e3486ef6ce88df8a183fc1cc2c65bfa9a53d3fd".to_string()));
    assert_eq!(wallet.get_public_address(), "hxb14e0c751899676a1a4e655a34063b42260f844b");

    Ok(())
}
