//! Activity 12: library logic behind a command-line report.

#![allow(unused_variables, unused_imports, dead_code)] // starter only: delete when complete
fn money(cents: u64) -> String {
    format!("${}.{:02}", cents / 100, cents % 100)
}

/// Builds the three-line stock report from CSV text.
pub fn run(csv: &str, threshold: u32) -> Result<String, String> {
    todo!("implement run (see README step 6)")
}
