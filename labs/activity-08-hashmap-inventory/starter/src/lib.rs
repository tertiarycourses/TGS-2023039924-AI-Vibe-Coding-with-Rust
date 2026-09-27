//! Activity 8: an inventory type wrapping a HashMap.

#![allow(unused_variables, unused_imports, dead_code)] // starter only: delete when complete
use std::collections::HashMap;

/// Stock levels keyed by SKU.
#[derive(Debug, Default)]
pub struct Inventory {
    stock: HashMap<String, u32>,
}

impl Inventory {
    /// Creates an empty inventory.
    pub fn new() -> Self {
        todo!("implement new (see README, step 7)")
    }

    /// Adds units; returns the new level, or `None` on overflow.
    pub fn receive(&mut self, sku: &str, qty: u32) -> Option<u32> {
        todo!("implement receive (see README, step 7)")
    }

    /// Removes units; unknown SKUs and shortages are errors.
    pub fn ship(&mut self, sku: &str, qty: u32) -> Result<u32, String> {
        todo!("implement ship (see README, step 7)")
    }

    /// Units on hand; zero for an unknown SKU.
    pub fn quantity(&self, sku: &str) -> u32 {
        todo!("implement quantity (see README, step 7)")
    }

    /// SKUs strictly below `threshold`, sorted by SKU.
    pub fn low_stock(&self, threshold: u32) -> Vec<(String, u32)> {
        todo!("implement low_stock (see README, step 7)")
    }
}
