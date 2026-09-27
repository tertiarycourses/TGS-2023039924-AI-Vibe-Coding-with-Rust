//! Activity 3: ownership, borrowing and mutable borrowing.

#![allow(unused_variables, unused_imports, dead_code)] // starter only: delete when complete
/// One stock record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    /// Product code.
    pub sku: String,
    /// Display name.
    pub name: String,
    /// Units on hand.
    pub qty: u32,
}

impl Item {
    /// Creates an owned item from borrowed text.
    pub fn new(sku: &str, name: &str, qty: u32) -> Self {
        todo!("implement new (see README step 6)")
    }
}

/// Borrows the items immutably; the caller keeps ownership.
pub fn total_quantity(items: &[Item]) -> u64 {
    todo!("implement total_quantity (see README step 6)")
}

/// Borrows one item mutably and returns its new quantity.
///
/// On overflow the item is left unchanged and `None` is returned.
pub fn restock(item: &mut Item, amount: u32) -> Option<u32> {
    todo!("implement restock (see README step 6)")
}

/// Takes ownership of the list and returns only the items in stock.
pub fn in_stock(items: Vec<Item>) -> Vec<Item> {
    todo!("implement in_stock (see README step 6)")
}
