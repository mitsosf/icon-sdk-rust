use icon_sdk::{icon_service, irc2};

#[tokio::test]
async fn test_name() -> Result<(), ()> {
    let icon_service = icon_service::IconService::new(Some("https://lisbon.net.solidwallet.io/api/v3".to_string()));
    let irc2 = irc2::IRC2::new(icon_service, "cx273548dff8bb77ffaac5a342c4c04aeae0bc48fa".to_string());
    let res = irc2.name().await;
    match res {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response["jsonrpc"], "2.0");
            assert!(!response.as_object().unwrap().contains_key("error"));
            assert_eq!(response["result"], "MyIRC2Token")
        },
        Err(e) => panic!("Error: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
async fn test_symbol() -> Result<(), ()> {
    let icon_service = icon_service::IconService::new(Some("https://lisbon.net.solidwallet.io/api/v3".to_string()));
    let irc2 = irc2::IRC2::new(icon_service, "cx273548dff8bb77ffaac5a342c4c04aeae0bc48fa".to_string());
    let res = irc2.symbol().await;
    match res {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response["jsonrpc"], "2.0");
            assert!(!response.as_object().unwrap().contains_key("error"));
            assert_eq!(response["result"], "MIT")
        },
        Err(e) => panic!("Error: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
async fn test_decimals() -> Result<(), ()> {
    let icon_service = icon_service::IconService::new(Some("https://lisbon.net.solidwallet.io/api/v3".to_string()));
    let irc2 = irc2::IRC2::new(icon_service, "cx273548dff8bb77ffaac5a342c4c04aeae0bc48fa".to_string());
    let res = irc2.decimals().await;
    match res {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response["jsonrpc"], "2.0");
            assert!(!response.as_object().unwrap().contains_key("error"));
            assert_eq!(response["result"], "0x12")
        },
        Err(e) => panic!("Error: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
async fn test_total_supply() -> Result<(), ()> {
    let icon_service = icon_service::IconService::new(Some("https://lisbon.net.solidwallet.io/api/v3".to_string()));
    let irc2 = irc2::IRC2::new(icon_service, "cx273548dff8bb77ffaac5a342c4c04aeae0bc48fa".to_string());
    let res = irc2.total_supply().await;
    match res {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response["jsonrpc"], "2.0");
            assert!(!response.as_object().unwrap().contains_key("error"));
            assert_eq!(response["result"], "0x3635c9adc5dea00000")
        },
        Err(e) => panic!("Error: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
async fn test_balance_of() -> Result<(), ()> {
    let icon_service = icon_service::IconService::new(Some("https://lisbon.net.solidwallet.io/api/v3".to_string()));
    let irc2 = irc2::IRC2::new(icon_service, "cx273548dff8bb77ffaac5a342c4c04aeae0bc48fa".to_string());
    let res = irc2.balance_of("hx8dc6ae3d93e60a2dddf80bfc5fb1cd16a2bf6160".to_string()).await;
    match res {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response["jsonrpc"], "2.0");
            assert!(!response.as_object().unwrap().contains_key("error"));
            assert!(response.as_object().unwrap().contains_key("result"));
        },
        Err(e) => panic!("Error: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
async fn test_transfer() -> Result<(), ()> {
    let wallet = icon_sdk::wallet::Wallet::new(Some("3468ea815d8896ef4552f10768caf2660689b965975c3ec2c1f5fe84bc3a77a5".to_string()));
    let icon_service = icon_service::IconService::new(Some("https://lisbon.net.solidwallet.io/api/v3".to_string()));
    let irc2 = irc2::IRC2::new(icon_service, "cx273548dff8bb77ffaac5a342c4c04aeae0bc48fa".to_string());
    let res = irc2.transfer(
        wallet,
        "hx8dc6ae3d93e60a2dddf80bfc5fb1cd16a2bf6160",
        "12.317",
        "0x3",
        "0x2",
        "0x1",
        "0x186a00"
    ).await;

    match res {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response["jsonrpc"], "2.0");
            assert!(!response.as_object().unwrap().contains_key("error"));
            assert!(response.as_object().unwrap().contains_key("result"));
        },
        Err(e) => panic!("Error: {:?}", e),
    }

    Ok(())
}
