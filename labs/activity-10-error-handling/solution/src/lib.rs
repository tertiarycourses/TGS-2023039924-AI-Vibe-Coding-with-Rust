//! Activity 10: recoverable errors with Result, ? and a custom error type.

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
    let fields: Vec<&str> = line.split(',').map(str::trim).collect();
    if fields.len() != 3 {
        return Err(ParseError::FieldCount(fields.len()));
    }
    if fields[0].is_empty() {
        return Err(ParseError::EmptySku);
    }
    Ok(Record {
        sku: fields[0].to_string(),
        qty: number("qty", fields[1])?,
        unit_cents: number("unit_cents", fields[2])?,
    })
}

/// Parses every line, keeping valid records and numbered errors.
pub fn parse_all(text: &str) -> (Vec<Record>, Vec<(usize, ParseError)>) {
    let mut records = Vec::new();
    let mut errors = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("sku,") {
            continue;
        }
        match parse_record(line) {
            Ok(record) => records.push(record),
            Err(error) => errors.push((index + 1, error)),
        }
    }
    (records, errors)
}
