// Sample: dbg! and assert_eq! as debugging tools.
fn total(values: &[u64]) -> u64 {
    values.iter().sum()
}

fn main() {
    let values = [35_960, 12_495, 0, 4_200];
    let result = dbg!(total(&values)); // prints file:line and value to stderr
    assert_eq!(result, 52_655, "total must include every line");
    let skipped: u64 = values.iter().skip(1).sum();
    println!("correct total {result}, total without the first line {skipped}");
    println!("difference {} = the first line", result - skipped);
}
