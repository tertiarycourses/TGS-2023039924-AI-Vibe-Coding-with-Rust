//! Activity 2: product-code validation with &str and String.

/// Returns true when `code`, after trimming, has the form `AAA-9999`.
pub fn is_valid_sku(code: &str) -> bool {
    let bytes = code.trim().as_bytes();
    bytes.len() == 8
        && bytes[..3].iter().all(u8::is_ascii_uppercase)
        && bytes[3] == b'-'
        && bytes[4..].iter().all(u8::is_ascii_digit)
}

/// Returns the trimmed, upper-case SKU when it is valid.
pub fn normalise_sku(code: &str) -> Option<String> {
    let upper = code.trim().to_ascii_uppercase();
    if is_valid_sku(&upper) {
        Some(upper)
    } else {
        None
    }
}
