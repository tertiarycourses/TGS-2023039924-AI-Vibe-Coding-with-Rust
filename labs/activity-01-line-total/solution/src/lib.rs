//! Activity 1: price one order line from a written specification.

/// Largest quantity a single order line may request.
pub const MAX_QTY: u32 = 10_000;

/// Returns the line total in cents.
///
/// Returns `None` when `qty` is outside `1..=MAX_QTY` or when the
/// multiplication would overflow `u64`.
pub fn line_total(unit_price_cents: u64, qty: u32) -> Option<u64> {
    if qty == 0 || qty > MAX_QTY {
        return None;
    }
    unit_price_cents.checked_mul(u64::from(qty))
}
