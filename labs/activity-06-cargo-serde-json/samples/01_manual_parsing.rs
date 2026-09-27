// Sample: parsing "sku,qty" by hand - the work serde automates for JSON.
struct Item {
    sku: String,
    qty: u32,
}

fn parse(line: &str) -> Result<Item, String> {
    let (sku, qty) = line.split_once(',').ok_or("expected sku,qty")?;
    let qty = qty.trim().parse::<u32>().map_err(|e| format!("qty: {e}"))?;
    Ok(Item {
        sku: sku.trim().to_string(),
        qty,
    })
}

fn main() {
    for line in ["ABC-0001, 40", "ABC-0002,-1", "no comma"] {
        match parse(line) {
            Ok(item) => {
                println!("{line:?} -> sku={} qty={}", item.sku, item.qty)
            }
            Err(error) => println!("{line:?} -> error: {error}"),
        }
    }
}
