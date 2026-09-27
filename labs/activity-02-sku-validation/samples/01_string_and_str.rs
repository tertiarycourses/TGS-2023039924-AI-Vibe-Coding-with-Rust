// Sample: owned String versus borrowed &str, and bytes versus characters.
fn describe(text: &str) {
    println!(
        "{text:?}: {} bytes, {} chars",
        text.len(),
        text.chars().count()
    );
}

fn main() {
    let owned: String = String::from("  abc-1234  ");
    let trimmed: &str = owned.trim(); // borrows part of `owned`
    let upper: String = trimmed.to_ascii_uppercase(); // new owned text
    describe(&owned);
    describe(trimmed);
    describe(&upper);
    describe("ÄBC-1234");
}
