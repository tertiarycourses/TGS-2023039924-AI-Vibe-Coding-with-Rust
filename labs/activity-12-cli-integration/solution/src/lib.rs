//! Activity 12: library logic behind a command-line report.

fn money(cents: u64) -> String {
    format!("${}.{:02}", cents / 100, cents % 100)
}

/// Builds the three-line stock report from CSV text.
pub fn run(csv: &str, threshold: u32) -> Result<String, String> {
    let mut count = 0usize;
    let mut total = 0u64;
    let mut low = Vec::new();
    for (index, raw) in csv.lines().enumerate() {
        let raw = raw.trim();
        if raw.is_empty() || raw.starts_with("sku,") {
            continue;
        }
        let fields: Vec<&str> = raw.split(',').map(str::trim).collect();
        let [sku, qty, unit] = fields.as_slice() else {
            return Err(format!("line {}: expected 3 fields", index + 1));
        };
        let qty: u32 = qty
            .parse()
            .map_err(|_| format!("line {}: bad qty", index + 1))?;
        let unit: u64 = unit
            .parse()
            .map_err(|_| format!("line {}: bad unit", index + 1))?;
        total = unit
            .checked_mul(u64::from(qty))
            .and_then(|value| total.checked_add(value))
            .ok_or("stock value overflowed u64")?;
        count += 1;
        if qty < threshold {
            low.push(sku.to_string());
        }
    }
    low.sort();
    let low = if low.is_empty() {
        "none".to_string()
    } else {
        low.join(", ")
    };
    Ok(format!(
        "ITEMS {count}\nTOTAL {}\nLOW STOCK {low}",
        money(total)
    ))
}
