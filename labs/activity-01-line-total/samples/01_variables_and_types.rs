// Sample: variables, mutability, constants and integer types.
const MAX_QTY: u32 = 10_000;

fn main() {
    let unit_price_cents: u64 = 250; // immutable by default
    let mut qty: u32 = 4; // mut: this value will change
    println!("price = {unit_price_cents} cents, qty = {qty}");
    qty += 1;
    println!("after adding one: qty = {qty} (max {MAX_QTY})");
    let total = unit_price_cents * u64::from(qty);
    println!(
        "total = {total} cents = ${}.{:02}",
        total / 100,
        total % 100
    );
    let big: u64 = u64::MAX;
    println!("checked_mul(2) on u64::MAX = {:?}", big.checked_mul(2));
}
