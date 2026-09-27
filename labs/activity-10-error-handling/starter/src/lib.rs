//! Activity 10: recoverable errors with Result, ? and a custom error type.

#![allow(unused_variables, unused_imports, dead_code)] // starter only: delete when complete
use std::fmt;

/// One valid stock record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    /// Product code.
    pub sku: String,
    /// Units on hand.
    pub qty: u32,
    /// Unit cost in cents.
    pub unit_cents: u64,
}

/// Why a line could not be parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// The line did not have exactly three fields.
    FieldCount(usize),
    /// The SKU field was empty.
    EmptySku,
    /// A numeric field was not a whole number.
    BadNumber {
        /// Field name.
        field: &'static str,
        /// Text that was found.
        value: String,
    },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::FieldCount(n) => {
                write!(f, "expected 3 fields, found {n}")
            }
            ParseError::EmptySku => write!(f, "SKU is empty"),
            ParseError::BadNumber { field, value } => {
                write!(f, "{field} is not a whole number: {value:?}")
            }
        }
    }
}

impl std::error::Error for ParseError {}

fn number<T: std::str::FromStr>(
    field: &'static str,
    value: &str,
) -> Result<T, ParseError> {
    value.parse().map_err(|_| ParseError::BadNumber {
        field,
        value: value.to_string(),
    })
}

/// Parses `sku,qty,unit_cents`; surrounding spaces are ignored.
pub fn parse_record(line: &str) -> Result<Record, ParseError> {
    todo!("implement parse_record (see README step 6)")
}

/// Parses every line, keeping valid records and numbered errors.
pub fn parse_all(text: &str) -> (Vec<Record>, Vec<(usize, ParseError)>) {
    todo!("implement parse_all (see README step 6)")
}
