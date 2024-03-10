use std::str::FromStr;
use rust_decimal::Decimal;
use icon_sdk_rust::icon_service;
use icon_sdk_rust::utils::helpers::hex_to_icx;

#[tokio::test]
async fn test_get_last_block() -> Result<(), ()> {
    let res = icon_service::get_last_block().await;
    match res {
        Ok(response) => {
            assert_eq!(response["jsonrpc"], "2.0");
            assert!(!response.as_object().unwrap().contains_key("error"));
            println!("{:?}", response);
        },
        Err(e) => println!("Error: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
async fn test_get_block_by_height() -> Result<(), ()> {
    let res = icon_service::get_block_by_height("0x0").await;
    match res {
        Ok(response) => {
            assert_eq!(response["jsonrpc"], "2.0");
            assert!(!response.as_object().unwrap().contains_key("error"));
            println!("{:?}", response);
        },
        Err(e) => println!("Error: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
async fn test_get_block_by_hash() -> Result<(), ()> {
    let res = icon_service::get_block_by_hash("0xcf43b3fd45981431a0e64f79d07bfcf703e064b73b802c5f32834eec72142190").await;
    match res {
        Ok(response) => {
            assert_eq!(response["jsonrpc"], "2.0");
            assert!(!response.as_object().unwrap().contains_key("error"));
            println!("{:?}", response);
        },
        Err(e) => println!("Error: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
async fn test_get_balance() -> Result<(), ()> {
    let res = icon_service::get_balance("hxd5ace539bf910635c2fa0e9c185d2d3c8d52c4cc").await;
    match res {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response["jsonrpc"], "2.0");
            assert!(!response.as_object().unwrap().contains_key("error"));
        },
        Err(e) => println!("Error: {:?}", e),
    }

    Ok(())
}
#[tokio::test]
async fn test_hex_to_icx() -> Result<(), ()> {
    let res = icon_sdk_rust::utils::helpers::hex_to_icx("0x63b5429420c741b16a10f");
    match res {
        Some(response) => {
            assert_eq!(response.to_string(), "7533727.039631672546337039");
        },
        None => println!("Error"),
    }

    Ok(())
}

#[tokio::test]
async fn test_icx_to_hex() -> Result<(), ()> {
    let res = icon_sdk_rust::utils::helpers::icx_to_hex(Decimal::from_str("7533727.039631672546337039").unwrap());
    match res {
        Some(response) => {
            assert_eq!(response, "0x63d8bac040145a956a22a");
            println!("{:?}", response);
        },
        None => println!("Error"),
    }

    Ok(())
}
