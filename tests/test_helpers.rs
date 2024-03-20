use std::str::FromStr;
use rust_decimal::Decimal;
use icon_sdk::utils::helpers;

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
