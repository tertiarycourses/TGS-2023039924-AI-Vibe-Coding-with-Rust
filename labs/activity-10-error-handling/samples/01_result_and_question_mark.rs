// Sample: Result and the ? operator.
use std::num::ParseIntError;

fn parse_qty(text: &str) -> Result<u32, ParseIntError> {
    text.trim().parse::<u32>()
}

fn total_qty(a: &str, b: &str) -> Result<u32, ParseIntError> {
    let first = parse_qty(a)?; // returns early on Err
    let second = parse_qty(b)?;
    Ok(first + second)
}

fn main() {
    println!("{:?}", total_qty("4", " 6 "));
    match total_qty("4", "six") {
        Ok(total) => println!("total {total}"),
        Err(error) => println!("error: {error}"),
    }
}
