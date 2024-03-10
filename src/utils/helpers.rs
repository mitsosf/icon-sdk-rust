use num_bigint::BigInt;
use num_traits::{Num, ToPrimitive};
use rust_decimal::Decimal;
use std::str::FromStr;

pub fn hex_to_icx(value: &str) -> Option<Decimal> {
    // Strip the "0x" prefix if present and parse the remaining hex string into a BigInt
    let clean_value = value.strip_prefix("0x").unwrap_or(value);
    let value_bigint = BigInt::from_str_radix(clean_value, 16).ok();

    // Convert BigInt to Decimal for arithmetic
    let decimal_value = Decimal::from_str(&value_bigint?.to_str_radix(10)).ok();

    // Create the divisor for 18 decimal places
    let divisor = Decimal::new(10i64.pow(18), 0);

    // Perform the division, scaling the result to 18 decimal places
    Some(decimal_value? / divisor)
}

pub fn icx_to_hex(value: Decimal) -> Option<String> {
    let multiplier = Decimal::from_str(&10u128.pow(18).to_string()).ok()?;
    // Perform the multiplication to adjust for decimal places
    let result_decimal = value * multiplier;
    let result_bigint = result_decimal.to_i128().and_then(|val| Some(BigInt::from(val)))?;

    // Convert BigInt to hexadecimal string and prefix with "0x"
    Some(format!("0x{}", result_bigint.to_str_radix(16)))
}
