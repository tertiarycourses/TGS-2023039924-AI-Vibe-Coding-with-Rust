use activity03::{in_stock, restock, total_quantity, Item};

fn sample() -> Vec<Item> {
    vec![
        Item::new("ABC-0001", "USB-C cable", 5),
        Item::new("ABC-0002", "Wall charger", 0),
        Item::new("ABC-0003", "Phone case", 7),
    ]
}

#[test]
fn total_borrows_without_taking_ownership() {
    let items = sample();
    assert_eq!(total_quantity(&items), 12);
    assert_eq!(items.len(), 3); // still usable: it was only borrowed
}

#[test]
fn empty_total_is_zero() {
    assert_eq!(total_quantity(&[]), 0);
}

#[test]
fn restock_changes_the_item_through_a_mutable_borrow() {
    let mut item = Item::new("ABC-0001", "USB-C cable", 5);
    assert_eq!(restock(&mut item, 3), Some(8));
    assert_eq!(item.qty, 8);
}

#[test]
fn restock_overflow_leaves_the_item_unchanged() {
    let mut item = Item::new("ABC-0001", "USB-C cable", u32::MAX);
    assert_eq!(restock(&mut item, 1), None);
    assert_eq!(item.qty, u32::MAX);
}

#[test]
fn in_stock_consumes_the_list_and_filters_it() {
    let kept = in_stock(sample());
    let skus: Vec<&str> = kept.iter().map(|item| item.sku.as_str()).collect();
    assert_eq!(skus, ["ABC-0001", "ABC-0003"]);
}
