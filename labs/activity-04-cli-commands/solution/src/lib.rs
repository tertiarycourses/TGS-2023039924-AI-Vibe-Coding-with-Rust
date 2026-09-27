//! Activity 4: a command parser produced by vibe coding and human review.

/// A validated StockPilot command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// Receive stock.
    Add {
        /// Product code.
        sku: String,
        /// Units received.
        qty: u32,
    },
    /// Remove stock.
    Remove {
        /// Product code.
        sku: String,
        /// Units removed.
        qty: u32,
    },
    /// List all stock.
    List,
}

/// Parses command-line words into a [`Command`].
pub fn parse_command(args: &[&str]) -> Result<Command, String> {
    match args {
        ["list"] => Ok(Command::List),
        [verb @ ("add" | "remove"), sku, qty] => {
            let qty: u32 = qty
                .parse()
                .map_err(|_| format!("invalid quantity: {qty}"))?;
            if qty == 0 {
                return Err("quantity must be at least 1".to_string());
            }
            let sku = sku.to_string();
            if *verb == "add" {
                Ok(Command::Add { sku, qty })
            } else {
                Ok(Command::Remove { sku, qty })
            }
        }
        _ => {
            Err("usage: add <sku> <qty> | remove <sku> <qty> | list"
                .to_string())
        }
    }
}
