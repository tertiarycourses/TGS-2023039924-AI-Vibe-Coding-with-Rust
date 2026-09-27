// Sample: closures and iterator chains instead of index loops.
fn main() {
    let prices = [899u64, 2499, 1299, 350];
    let threshold = 1000;
    let expensive: Vec<u64> =
        prices.iter().copied().filter(|p| *p >= threshold).collect();
    println!("at least {threshold}: {expensive:?}");
    let doubled: Vec<u64> = prices.iter().map(|p| p * 2).collect();
    println!("doubled: {doubled:?}");
    let total = prices.iter().try_fold(0u64, |acc, p| acc.checked_add(*p));
    println!("checked total: {total:?}");
    let add_gst = |cents: u64| cents * 109 / 100;
    println!("899 with 9% GST: {}", add_gst(899));
}
