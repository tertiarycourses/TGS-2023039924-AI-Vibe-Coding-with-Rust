//! Reorder suggestions designed and documented with AI assistance.
//!
//! See `docs/design.md` for the requirement trace (R1-R4).
#![deny(missing_docs)]
#![allow(unused_variables, unused_imports, dead_code)] // starter only: delete when complete

/// One stock-keeping unit and its planning inputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StockItem {
    /// Product code, for example `ABC-0001`.
    pub sku: String,
    /// Units currently on hand.
    pub on_hand: u32,
    /// Average units used per day.
    pub daily_usage: u32,
    /// Supplier lead time in days.
    pub lead_days: u32,
    /// Buffer stock kept for demand spikes.
    pub safety_stock: u32,
}

/// A proposed purchase-order line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Suggestion {
    /// Product code.
    pub sku: String,
    /// Units to order.
    pub order_qty: u32,
}

/// Days of usage the order should cover beyond the reorder point (R3).
pub const REVIEW_DAYS: u32 = 7;

/// Suggests purchase quantities for items at or below their reorder point.
///
/// * R1: reorder point = `daily_usage x lead_days + safety_stock`.
/// * R2: items above their reorder point are not suggested.
/// * R3: order quantity = reorder point + [`REVIEW_DAYS`] of usage - on hand;
///   zero quantities are not suggested.
/// * R4: items whose arithmetic overflows are skipped, never wrapped.
///
/// Suggestions are sorted by SKU.
///
/// # Examples
///
/// ```
/// use activity14::{reorder_suggestions, StockItem};
/// let item = StockItem {
///     sku: "ABC-0001".into(),
///     on_hand: 50,
///     daily_usage: 10,
///     lead_days: 5,
///     safety_stock: 20,
/// };
/// assert_eq!(reorder_suggestions(&[item])[0].order_qty, 90);
/// ```
pub fn reorder_suggestions(items: &[StockItem]) -> Vec<Suggestion> {
    todo!("implement reorder_suggestions (see README, step 7)")
}
