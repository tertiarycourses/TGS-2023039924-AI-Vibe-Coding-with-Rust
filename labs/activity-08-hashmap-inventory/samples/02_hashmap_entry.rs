// Sample: counting stock with HashMap::entry, then sorting for stable output.
use std::collections::HashMap;

fn main() {
    let deliveries = [("ABC-0001", 5), ("ABC-0002", 3), ("ABC-0001", 2)];
    let mut stock: HashMap<&str, u32> = HashMap::new();
    for (sku, qty) in deliveries {
        *stock.entry(sku).or_insert(0) += qty;
    }
    let mut rows: Vec<(&str, u32)> =
        stock.iter().map(|(s, q)| (*s, *q)).collect();
    rows.sort(); // HashMap order is not defined: sort before printing
    println!("{rows:?}");
    match stock.get("ZZZ-9999") {
        Some(qty) => println!("found {qty}"),
        None => println!("ZZZ-9999 is not stocked"),
    }
}
