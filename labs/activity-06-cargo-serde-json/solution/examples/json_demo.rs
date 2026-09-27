//! Run from solution/: cargo run --example json_demo
use activity06::{load_items, stock_value, to_json};

fn main() {
    let json = r#"[{"sku":"ABC-0001","name":"USB-C cable","qty":40,"unit_cents":899}]"#;
    let items = load_items(json).expect("valid JSON");
    println!(
        "loaded {} item(s); value = {:?} cents",
        items.len(),
        stock_value(&items)
    );
    println!("{}", to_json(&items).expect("serialisable"));
    let bad = r#"[{"sku":"ABC-0002","name":"Hub","qty":-1,"unit_cents":10}]"#;
    println!("negative qty -> {}", load_items(bad).unwrap_err());
}
