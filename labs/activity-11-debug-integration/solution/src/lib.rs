//! Activity 11: an integrated stock summary (the starter has two seeded defects).

/// One parsed stock line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
    /// Product code.
    pub sku: String,
    /// Units on hand.
    pub qty: u32,
    /// Unit cost in cents.
    pub unit_cents: u64,
}

/// Result of the nightly summary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Summary {
    /// Number of stock lines.
    pub lines: usize,
    /// Total stock value in cents.
    pub total_cents: u64,
    /// SKUs strictly below the threshold, sorted.
    pub low_stock: Vec<String>,
}

/// Parses `sku,qty,unit_cents` lines, skipping the header and blanks.
pub fn parse_lines(csv: &str) -> Result<Vec<Line>, String> {
    let mut lines = Vec::new();
    for (index, raw) in csv.lines().enumerate() {
        let raw = raw.trim();
        if raw.is_empty() || raw.starts_with("sku,") {
            continue;
        }
        let fields: Vec<&str> = raw.split(',').map(str::trim).collect();
        let [sku, qty, unit] = fields.as_slice() else {
            return Err(format!("line {}: expected 3 fields", index + 1));
        };
        let qty = qty
            .parse()
            .map_err(|_| format!("line {}: bad qty {qty:?}", index + 1))?;
        let unit_cents = unit
            .parse()
            .map_err(|_| format!("line {}: bad unit {unit:?}", index + 1))?;
        lines.push(Line {
            sku: sku.to_string(),
            qty,
            unit_cents,
        });
    }
    Ok(lines)
}

/// Total value of every line, or `None` on overflow.
pub fn total_cents(lines: &[Line]) -> Option<u64> {
    lines.iter().try_fold(0u64, |total, line| {
        total.checked_add(line.unit_cents.checked_mul(u64::from(line.qty))?)
    })
}

/// SKUs strictly below `threshold`, sorted.
pub fn low_stock(lines: &[Line], threshold: u32) -> Vec<String> {
    let mut skus: Vec<String> = lines
        .iter()
        .filter(|line| line.qty < threshold)
        .map(|line| line.sku.clone())
        .collect();
    skus.sort();
    skus
}

/// Parses, totals and flags low stock in one call.
pub fn summarise(csv: &str, threshold: u32) -> Result<Summary, String> {
    let lines = parse_lines(csv)?;
    let total_cents =
        total_cents(&lines).ok_or("stock value overflowed u64")?;
    Ok(Summary {
        lines: lines.len(),
        total_cents,
        low_stock: low_stock(&lines, threshold),
    })
}
