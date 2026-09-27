// Sample: validating a product code character by character.
fn is_valid_sku(code: &str) -> bool {
    let chars: Vec<char> = code.trim().chars().collect();
    chars.len() == 8
        && chars[..3].iter().all(|c| c.is_ascii_uppercase())
        && chars[3] == '-'
        && chars[4..].iter().all(|c| c.is_ascii_digit())
}

fn main() {
    for code in [
        "ABC-1234",
        " ABC-1234\n",
        "abc-1234",
        "ABC-12A4",
        "ÄBC-1234",
    ] {
        println!("{code:?} valid = {}", is_valid_sku(code));
    }
}
