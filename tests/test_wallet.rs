use icon_sdk::wallet::Wallet;

#[tokio::test]
async fn test_wallet() -> Result<(), ()> {
    let wallet = Wallet::new(Some("f4ade1ff528c9e0bf10d35909e3486ef6ce88df8a183fc1cc2c65bfa9a53d3fd".to_string()));
    assert_eq!(wallet.get_public_address(), "hxb14e0c751899676a1a4e655a34063b42260f844b");

    Ok(())
}
