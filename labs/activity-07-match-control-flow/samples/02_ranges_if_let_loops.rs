// Sample: match on ranges, if let, and a while loop.
fn status(qty: u32) -> &'static str {
    match qty {
        0 => "out of stock",
        1..=4 => "low",
        5..=99 => "ok",
        _ => "bulk",
    }
}

fn main() {
    for qty in [0, 3, 5, 99, 100] {
        println!("{qty:>3} -> {}", status(qty));
    }
    let found: Option<u32> = Some(7);
    if let Some(qty) = found {
        println!("if let found {qty}");
    }
    let mut countdown = 3;
    while countdown > 0 {
        print!("{countdown} ");
        countdown -= 1;
    }
    println!("go");
}
