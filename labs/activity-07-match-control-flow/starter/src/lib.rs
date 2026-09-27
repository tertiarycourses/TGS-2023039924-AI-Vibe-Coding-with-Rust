//! Activity 7: control flow with enums, match and early return.

#![allow(unused_variables, unused_imports, dead_code)] // starter only: delete when complete
/// One stock movement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Movement {
    /// Units received from a supplier.
    Receive(u32),
    /// Units shipped to a customer.
    Ship(u32),
    /// Signed stock-take correction.
    Adjust(i64),
}

/// Why a movement was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StockError {
    /// A shipment asked for more than is available.
    Insufficient {
        /// Units on hand.
        available: u32,
        /// Units requested.
        requested: u32,
    },
    /// The result would exceed u32::MAX.
    Overflow,
    /// The result would be below zero.
    NegativeResult,
}

/// Applies one movement and returns the new stock level.
pub fn apply(stock: u32, movement: Movement) -> Result<u32, StockError> {
    todo!("implement apply (see README step 6)")
}

/// Applies movements in order and stops at the first error.
pub fn apply_all(
    stock: u32,
    movements: &[Movement],
) -> Result<u32, StockError> {
    todo!("implement apply_all (see README step 6)")
}
