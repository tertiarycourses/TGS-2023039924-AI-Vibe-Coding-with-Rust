use activity06::{load_items, stock_value, to_json, Item};

const SAMPLE: &str = include_str!("../../data/inventory.json");

#[test]
fn loads_the_sample_file() {
    let items = load_items(SAMPLE).expect("sample is valid");
    assert_eq!(items.len(), 3);
    assert_eq!(items[1].name, "Wall charger");
}

#[test]
fn calculates_stock_value() {
    let items = load_items(SAMPLE).unwrap();
    assert_eq!(stock_value(&items), Some(65_948));
}

#[test]
fn missing_field_is_rejected() {
    assert!(load_items(r#"[{"sku":"ABC-0009","name":"Hub","qty":1}]"#).is_err());
}

#[test]
fn negative_quantity_is_rejected_by_the_type() {
    let json = r#"[{"sku":"ABC-0009","name":"Hub","qty":-1,"unit_cents":10}]"#;
    assert!(load_items(json).is_err());
}

#[test]
fn json_round_trip_preserves_items() {
    let items = load_items(SAMPLE).unwrap();
    let again = load_items(&to_json(&items).unwrap()).unwrap();
    assert_eq!(items, again);
}

#[test]
fn stock_value_overflow_is_none() {
    let item = Item {
        sku: "X".into(),
        name: "Bulk".into(),
        qty: 2,
        unit_cents: u64::MAX,
    };
    assert_eq!(stock_value(&[item]), None);
}
