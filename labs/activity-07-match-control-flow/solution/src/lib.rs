//! Activity 7: control flow with enums, match and early return.

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
    match movement {
        Movement::Receive(qty) => {
            stock.checked_add(qty).ok_or(StockError::Overflow)
        }
        Movement::Ship(qty) if qty > stock => Err(StockError::Insufficient {
            available: stock,
            requested: qty,
        }),
        Movement::Ship(qty) => Ok(stock - qty),
        Movement::Adjust(delta) => {
            let next = i64::from(stock)
                .checked_add(delta)
                .ok_or(StockError::Overflow)?;
            if next < 0 {
                return Err(StockError::NegativeResult);
            }
            u32::try_from(next).map_err(|_| StockError::Overflow)
        }
    }
}

/// Applies movements in order and stops at the first error.
pub fn apply_all(
    stock: u32,
    movements: &[Movement],
) -> Result<u32, StockError> {
    let mut current = stock;
    for movement in movements {
        current = apply(current, *movement)?;
    }
    Ok(current)
}
