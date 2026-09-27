// Sample: a custom error type with Display, used with ? in main.
use std::fmt;

#[derive(Debug)]
enum StockError {
    Unknown(String),
    Insufficient { available: u32, requested: u32 },
}

impl fmt::Display for StockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StockError::Unknown(sku) => write!(f, "unknown SKU {sku}"),
            StockError::Insufficient {
                available,
                requested,
            } => {
                write!(
                    f,
                    "requested {requested} but only {available} available"
                )
            }
        }
    }
}

impl std::error::Error for StockError {}

fn ship(sku: &str, available: u32, requested: u32) -> Result<u32, StockError> {
    if sku != "ABC-0001" {
        return Err(StockError::Unknown(sku.to_string()));
    }
    if requested > available {
        return Err(StockError::Insufficient {
            available,
            requested,
        });
    }
    Ok(available - requested)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("left: {}", ship("ABC-0001", 5, 2)?);
    for (sku, requested) in [("ZZZ-9999", 1), ("ABC-0001", 9)] {
        if let Err(error) = ship(sku, 5, requested) {
            println!("error: {error}");
        }
    }
    Ok(())
}
