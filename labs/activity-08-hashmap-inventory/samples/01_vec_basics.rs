// Sample: a growable Vec, sorting, safe access with get.
fn main() {
    let mut skus: Vec<String> = Vec::new();
    skus.push("CCC-0003".to_string());
    skus.push("AAA-0001".to_string());
    skus.push("BBB-0002".to_string());
    skus.sort();
    println!("sorted: {skus:?}");
    println!("first: {:?}, tenth: {:?}", skus.first(), skus.get(9));
    for (i, sku) in skus.iter().enumerate() {
        println!("{}. {sku}", i + 1);
    }
}
