//! Sample library: document an API so rustdoc can render it and test it.

/// Days of usage kept as safety stock.
pub const SAFETY_DAYS: u32 = 2;

/// Returns the safety stock in units for a daily usage.
///
/// # Examples
///
/// ```
/// assert_eq!(docsample::safety_stock(12), Some(24));
/// assert_eq!(docsample::safety_stock(u32::MAX), None);
/// ```
pub fn safety_stock(daily_usage: u32) -> Option<u32> {
    daily_usage.checked_mul(SAFETY_DAYS)
}
