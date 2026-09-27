// Sample: writing JSON by hand with Display - fragile, which is why we use serde.
use std::fmt;

struct Item {
    sku: String,
    name: String,
    qty: u32,
    unit_cents: u64,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"{{"sku":"{}","name":"{}","qty":{},"unit_cents":{}}}"#,
            self.sku, self.name, self.qty, self.unit_cents
        )
    }
}

fn main() {
    let item = Item {
        sku: "ABC-0001".into(),
        name: "USB-C cable".into(),
        qty: 40,
        unit_cents: 899,
    };
    println!("{item}");
    println!("Hand-written JSON breaks if a name contains a quote; serde escapes it for you.");
}
