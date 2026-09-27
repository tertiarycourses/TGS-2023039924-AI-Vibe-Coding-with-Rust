use activity14::{reorder_suggestions, StockItem, Suggestion};

fn item(
    sku: &str,
    on_hand: u32,
    daily_usage: u32,
    lead_days: u32,
    safety_stock: u32,
) -> StockItem {
    StockItem {
        sku: sku.into(),
        on_hand,
        daily_usage,
        lead_days,
        safety_stock,
    }
}

fn suggestion(sku: &str, order_qty: u32) -> Suggestion {
    Suggestion {
        sku: sku.into(),
        order_qty,
    }
}

#[test]
fn r1_r3_below_the_point_orders_up_to_target() {
    assert_eq!(
        reorder_suggestions(&[item("ABC-0001", 50, 10, 5, 20)]),
        [suggestion("ABC-0001", 90)]
    );
}

#[test]
fn r2_above_the_point_is_not_suggested() {
    assert!(reorder_suggestions(&[item("ABC-0002", 71, 10, 5, 20)]).is_empty());
}

#[test]
fn r2_exactly_at_the_point_is_suggested() {
    assert_eq!(
        reorder_suggestions(&[item("ABC-0003", 70, 10, 5, 20)]),
        [suggestion("ABC-0003", 70)]
    );
}

#[test]
fn r3_zero_quantity_is_not_suggested() {
    assert!(reorder_suggestions(&[item("ABC-0005", 0, 0, 10, 0)]).is_empty());
}

#[test]
fn r4_overflow_is_skipped_without_panic() {
    let items = [
        item("ABC-0006", 0, u32::MAX, 2, 0),
        item("ABC-0007", 0, 1, 1, 0),
    ];
    assert_eq!(reorder_suggestions(&items), [suggestion("ABC-0007", 8)]);
}

#[test]
fn output_is_sorted_by_sku() {
    let items = [item("ZZZ-0001", 0, 1, 1, 0), item("AAA-0001", 0, 1, 1, 0)];
    let skus: Vec<String> = reorder_suggestions(&items)
        .into_iter()
        .map(|s| s.sku)
        .collect();
    assert_eq!(skus, ["AAA-0001", "ZZZ-0001"]);
}
