//! Activity 1: price one order line from a written specification.

#![allow(unused_variables, unused_imports, dead_code)] // starter only: delete when complete
/// Largest quantity a single order line may request.
pub const MAX_QTY: u32 = 10_000;

/// Returns the line total in cents.
///
/// Returns `None` when `qty` is outside `1..=MAX_QTY` or when the
/// multiplication would overflow `u64`.
pub fn line_total(unit_price_cents: u64, qty: u32) -> Option<u64> {
    todo!("implement line_total (see README, step 7)")
}
