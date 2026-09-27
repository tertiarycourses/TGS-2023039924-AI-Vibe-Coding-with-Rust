//! Activity 6: a JSON data contract using the serde and serde_json crates.

#![allow(unused_variables, unused_imports, dead_code)] // starter only: delete when complete
use serde::{Deserialize, Serialize};

/// One inventory record as exported by purchasing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    /// Product code.
    pub sku: String,
    /// Display name.
    pub name: String,
    /// Units on hand.
    pub qty: u32,
    /// Unit cost in cents.
    pub unit_cents: u64,
}

/// Parses a JSON array of items; missing or mistyped fields are errors.
pub fn load_items(json: &str) -> Result<Vec<Item>, serde_json::Error> {
    todo!("implement load_items (see README, step 7)")
}

/// Serialises items as pretty-printed JSON.
pub fn to_json(items: &[Item]) -> Result<String, serde_json::Error> {
    todo!("implement to_json (see README, step 7)")
}

/// Total stock value in cents, or `None` on overflow.
pub fn stock_value(items: &[Item]) -> Option<u64> {
    todo!("implement stock_value (see README, step 7)")
}
