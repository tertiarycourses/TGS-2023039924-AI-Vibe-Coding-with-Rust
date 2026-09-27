use activity09::{
    mixed_total, names_at_least, order_total, Priced, Product, Service,
};

fn products() -> Vec<Product> {
    vec![
        Product {
            name: "Cable".into(),
            unit_cents: 899,
            qty: 2,
        },
        Product {
            name: "Charger".into(),
            unit_cents: 2499,
            qty: 1,
        },
    ]
}

fn setup() -> Service {
    Service {
        description: "Device set-up".into(),
        rate_cents: 5000,
        hours: 2,
    }
}

#[test]
fn default_method_prices_a_line() {
    assert_eq!(setup().line_cents(), Some(10_000));
}

#[test]
fn generic_total_for_one_type() {
    assert_eq!(order_total(&products()), Some(4_297));
    assert_eq!(order_total::<Product>(&[]), Some(0));
}

#[test]
fn trait_objects_mix_types() {
    let items = products();
    let service = setup();
    let lines: Vec<&dyn Priced> = vec![&items[0], &items[1], &service];
    assert_eq!(mixed_total(&lines), Some(14_297));
}

#[test]
fn overflow_is_none() {
    let bulk = Product {
        name: "Bulk".into(),
        unit_cents: u64::MAX,
        qty: 2,
    };
    assert_eq!(order_total(&[bulk]), None);
}

#[test]
fn filter_and_map_keep_input_order() {
    assert_eq!(names_at_least(&products(), 1_000), ["Charger"]);
    assert_eq!(names_at_least(&products(), 0), ["Cable", "Charger"]);
}
