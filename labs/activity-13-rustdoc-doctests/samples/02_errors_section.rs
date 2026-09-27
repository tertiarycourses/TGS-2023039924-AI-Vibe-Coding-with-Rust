//! Sample library: documenting a function that returns Result.

/// Parses a whole-number quantity.
///
/// # Errors
///
/// Returns a message when `text` is not a whole number.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), String> {
/// let qty = docsample2::parse_qty(" 7 ")?;
/// assert_eq!(qty, 7);
/// assert!(docsample2::parse_qty("seven").is_err());
/// # Ok(())
/// # }
/// ```
pub fn parse_qty(text: &str) -> Result<u32, String> {
    text.trim()
        .parse()
        .map_err(|_| format!("not a whole number: {text}"))
}
