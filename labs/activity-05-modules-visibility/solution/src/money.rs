//! Currency formatting, kept separate from reporting.

/// Formats whole cents as dollars, for example `1205` becomes `"$12.05"`.
pub fn format_cents(cents: u64) -> String {
    format!("${}.{:02}", cents / 100, cents % 100)
}
