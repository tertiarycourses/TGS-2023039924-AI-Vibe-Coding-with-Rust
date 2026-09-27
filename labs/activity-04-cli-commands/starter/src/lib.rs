//! Activity 4: a command parser produced by vibe coding and human review.

#![allow(unused_variables, unused_imports, dead_code)] // starter only: delete when complete
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
    todo!("implement parse_command (see README step 6)")
}
