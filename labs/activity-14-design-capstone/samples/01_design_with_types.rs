// Sample: newtypes make invalid values hard to create.
struct Units(u32);

struct Sku(String);

impl Sku {
    fn parse(text: &str) -> Option<Sku> {
        let valid = text.len() == 8 && text.as_bytes()[3] == b'-';
        valid.then(|| Sku(text.to_string()))
    }
}

fn order(sku: &Sku, qty: Units) -> String {
    format!("order {} x {}", qty.0, sku.0)
}

fn main() {
    match Sku::parse("ABC-0001") {
        Some(sku) => println!("{}", order(&sku, Units(90))),
        None => println!("invalid SKU"),
    }
    println!("bad SKU accepted? {}", Sku::parse("bad").is_some());
}
