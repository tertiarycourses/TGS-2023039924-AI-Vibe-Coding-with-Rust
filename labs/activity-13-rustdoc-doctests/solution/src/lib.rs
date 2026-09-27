//! Reorder-point calculations for the StockPilot stock service.
//!
//! Requirement trace: R1 reorder-point formula, R2 overflow is reported
//! instead of wrapping, R3 stock at or below the point needs reordering.
#![deny(missing_docs)]

/// Calculates the reorder point in units.
///
/// `reorder point = daily_usage x lead_days + safety_stock` (R1).
///
/// # Examples
///
/// ```
/// use activity13::reorder_point;
/// assert_eq!(reorder_point(12, 5, 20), Some(80));
/// ```
///
/// Returns `None` instead of wrapping when the result exceeds `u32` (R2):
///
/// ```
/// use activity13::reorder_point;
/// assert_eq!(reorder_point(u32::MAX, 2, 0), None);
/// ```
pub fn reorder_point(
    daily_usage: u32,
    lead_days: u32,
    safety_stock: u32,
) -> Option<u32> {
    daily_usage
        .checked_mul(lead_days)?
        .checked_add(safety_stock)
}

/// Reports whether stock on hand has reached the reorder point (R3).
///
/// Stock exactly at the reorder point needs reordering.
///
/// # Examples
///
/// ```
/// use activity13::needs_reorder;
/// assert!(needs_reorder(80, 80));
/// assert!(!needs_reorder(81, 80));
/// ```
pub fn needs_reorder(on_hand: u32, reorder_point: u32) -> bool {
    on_hand <= reorder_point
}
