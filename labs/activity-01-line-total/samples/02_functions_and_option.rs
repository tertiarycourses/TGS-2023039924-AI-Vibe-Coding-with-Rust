// Sample: a function whose return type says "this can be refused".
fn line_total(unit_price_cents: u64, qty: u32) -> Option<u64> {
    if qty == 0 || qty > 10_000 {
        return None;
    }
    unit_price_cents.checked_mul(u64::from(qty))
}

fn main() {
    for (price, qty) in [(250, 4), (250, 0), (1, 10_001), (u64::MAX, 2)] {
        match line_total(price, qty) {
            Some(total) => println!("{qty} x {price} = {total}"),
            None => println!("{qty} x {price} rejected"),
        }
    }
}
