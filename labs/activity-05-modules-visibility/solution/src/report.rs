//! Report lines built on top of the money module.

use crate::money::format_cents;

/// Builds `"<name> x<qty> @ <unit> = <total>"`, or `None` on overflow.
pub fn report_line(name: &str, qty: u32, unit_cents: u64) -> Option<String> {
    let total = unit_cents.checked_mul(u64::from(qty))?;
    Some(format!(
        "{name} x{qty} @ {} = {}",
        format_cents(unit_cents),
        format_cents(total)
    ))
}

/// Joins one report line per row; any overflow fails the whole report.
pub fn report(rows: &[(&str, u32, u64)]) -> Option<String> {
    let lines: Option<Vec<String>> = rows
        .iter()
        .map(|&(name, qty, unit)| report_line(name, qty, unit))
        .collect();
    lines.map(|lines| lines.join("\n"))
}
