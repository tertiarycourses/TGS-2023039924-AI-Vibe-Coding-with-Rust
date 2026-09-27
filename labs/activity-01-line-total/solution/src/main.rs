use activity01::line_total;

fn main() {
    let unit_price_cents: u64 = 250;
    let qty: u32 = 4;
    match line_total(unit_price_cents, qty) {
        Some(total) => {
            println!("{qty} x {unit_price_cents} cents = {total} cents")
        }
        None => println!("rejected: quantity or total out of range"),
    }
}
