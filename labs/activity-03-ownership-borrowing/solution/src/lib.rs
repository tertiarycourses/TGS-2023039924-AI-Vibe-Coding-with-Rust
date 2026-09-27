//! Activity 3: ownership, borrowing and mutable borrowing.

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
        Item {
            sku: sku.to_string(),
            name: name.to_string(),
            qty,
        }
    }
}

/// Borrows the items immutably; the caller keeps ownership.
pub fn total_quantity(items: &[Item]) -> u64 {
    items.iter().map(|item| u64::from(item.qty)).sum()
}

/// Borrows one item mutably and returns its new quantity.
///
/// On overflow the item is left unchanged and `None` is returned.
pub fn restock(item: &mut Item, amount: u32) -> Option<u32> {
    let new_qty = item.qty.checked_add(amount)?;
    item.qty = new_qty;
    Some(new_qty)
}

/// Takes ownership of the list and returns only the items in stock.
pub fn in_stock(items: Vec<Item>) -> Vec<Item> {
    items.into_iter().filter(|item| item.qty > 0).collect()
}
