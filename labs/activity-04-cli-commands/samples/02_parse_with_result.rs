// Sample: turning text into a number safely with Result and ?.
fn parse_qty(text: &str) -> Result<u32, String> {
    let qty: u32 = text
        .parse()
        .map_err(|_| format!("not a whole number: {text}"))?;
    if qty == 0 {
        return Err("quantity must be at least 1".to_string());
    }
    Ok(qty)
}

fn main() {
    for input in ["5", "0", "five", "-3"] {
        match parse_qty(input) {
            Ok(qty) => println!("{input:>5} -> Ok({qty})"),
            Err(message) => println!("{input:>5} -> Err({message})"),
        }
    }
}
