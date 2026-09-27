#![warn(missing_docs)] // TODO: document every public item

pub fn reorder_point(
    daily_usage: u32,
    lead_days: u32,
    safety_stock: u32,
) -> Option<u32> {
    daily_usage
        .checked_mul(lead_days)?
        .checked_add(safety_stock)
}

pub fn needs_reorder(on_hand: u32, reorder_point: u32) -> bool {
    on_hand <= reorder_point
}
