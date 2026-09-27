//! Activity 8: an inventory type wrapping a HashMap.

use std::collections::HashMap;

/// Stock levels keyed by SKU.
#[derive(Debug, Default)]
pub struct Inventory {
    stock: HashMap<String, u32>,
}

impl Inventory {
    /// Creates an empty inventory.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds units; returns the new level, or `None` on overflow.
    pub fn receive(&mut self, sku: &str, qty: u32) -> Option<u32> {
        let level = self.stock.entry(sku.to_string()).or_insert(0);
        *level = level.checked_add(qty)?;
        Some(*level)
    }

    /// Removes units; unknown SKUs and shortages are errors.
    pub fn ship(&mut self, sku: &str, qty: u32) -> Result<u32, String> {
        let level = self
            .stock
            .get_mut(sku)
            .ok_or_else(|| format!("unknown SKU {sku}"))?;
        if qty > *level {
            return Err(format!("only {level} of {sku} available"));
        }
        *level -= qty;
        Ok(*level)
    }

    /// Units on hand; zero for an unknown SKU.
    pub fn quantity(&self, sku: &str) -> u32 {
        self.stock.get(sku).copied().unwrap_or(0)
    }

    /// SKUs strictly below `threshold`, sorted by SKU.
    pub fn low_stock(&self, threshold: u32) -> Vec<(String, u32)> {
        let mut low: Vec<(String, u32)> = self
            .stock
            .iter()
            .filter(|(_, qty)| **qty < threshold)
            .map(|(sku, qty)| (sku.clone(), *qty))
            .collect();
        low.sort();
        low
    }
}
