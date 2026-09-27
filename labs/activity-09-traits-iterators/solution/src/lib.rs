//! Activity 9: traits, generics, trait objects and iterator chains.

/// Anything that can be priced as quantity x unit price.
pub trait Priced {
    /// Price of one unit in cents.
    fn unit_cents(&self) -> u64;
    /// Number of units.
    fn quantity(&self) -> u32;
    /// Line value in cents; `None` on overflow. Shared by every implementor.
    fn line_cents(&self) -> Option<u64> {
        self.unit_cents().checked_mul(u64::from(self.quantity()))
    }
}

/// A physical product line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Product {
    /// Display name.
    pub name: String,
    /// Unit price in cents.
    pub unit_cents: u64,
    /// Units ordered.
    pub qty: u32,
}

/// A billable service line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Service {
    /// What is being done.
    pub description: String,
    /// Hourly rate in cents.
    pub rate_cents: u64,
    /// Hours billed.
    pub hours: u32,
}

impl Priced for Product {
    fn unit_cents(&self) -> u64 {
        self.unit_cents
    }
    fn quantity(&self) -> u32 {
        self.qty
    }
}

impl Priced for Service {
    fn unit_cents(&self) -> u64 {
        self.rate_cents
    }
    fn quantity(&self) -> u32 {
        self.hours
    }
}

/// Totals lines of one type (static dispatch).
pub fn order_total<T: Priced>(lines: &[T]) -> Option<u64> {
    lines
        .iter()
        .try_fold(0u64, |total, line| total.checked_add(line.line_cents()?))
}

/// Totals lines of different types (dynamic dispatch).
pub fn mixed_total(lines: &[&dyn Priced]) -> Option<u64> {
    lines
        .iter()
        .try_fold(0u64, |total, line| total.checked_add(line.line_cents()?))
}

/// Names of products priced at or above `min_cents`, in input order.
pub fn names_at_least(products: &[Product], min_cents: u64) -> Vec<&str> {
    products
        .iter()
        .filter(|product| product.unit_cents >= min_cents)
        .map(|product| product.name.as_str())
        .collect()
}
