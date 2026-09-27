use activity08::Inventory;

#[test]
fn unknown_sku_has_zero_quantity() {
    assert_eq!(Inventory::new().quantity("ABC-0001"), 0);
}

#[test]
fn receive_accumulates() {
    let mut inv = Inventory::new();
    assert_eq!(inv.receive("ABC-0001", 5), Some(5));
    assert_eq!(inv.receive("ABC-0001", 3), Some(8));
    assert_eq!(inv.quantity("ABC-0001"), 8);
}

#[test]
fn receive_overflow_leaves_stock_unchanged() {
    let mut inv = Inventory::new();
    inv.receive("ABC-0001", u32::MAX);
    assert_eq!(inv.receive("ABC-0001", 1), None);
    assert_eq!(inv.quantity("ABC-0001"), u32::MAX);
}

#[test]
fn ship_reduces_stock() {
    let mut inv = Inventory::new();
    inv.receive("ABC-0001", 5);
    assert_eq!(inv.ship("ABC-0001", 2), Ok(3));
}

#[test]
fn ship_errors_leave_stock_unchanged() {
    let mut inv = Inventory::new();
    inv.receive("ABC-0001", 5);
    assert!(inv.ship("ZZZ-9999", 1).unwrap_err().contains("unknown SKU"));
    assert!(inv.ship("ABC-0001", 6).is_err());
    assert_eq!(inv.quantity("ABC-0001"), 5);
}

#[test]
fn low_stock_is_strict_and_sorted() {
    let mut inv = Inventory::new();
    inv.receive("CCC-0003", 5);
    inv.receive("BBB-0002", 3);
    inv.receive("AAA-0001", 0);
    assert_eq!(
        inv.low_stock(5),
        vec![("AAA-0001".to_string(), 0), ("BBB-0002".to_string(), 3)]
    );
}
