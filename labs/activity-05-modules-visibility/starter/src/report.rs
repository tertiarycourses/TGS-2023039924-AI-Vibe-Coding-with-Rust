//! Report lines built on top of the money module.

#![allow(unused_variables, unused_imports, dead_code)] // starter only: delete when complete
use crate::money::format_cents;

/// Builds `"<name> x<qty> @ <unit> = <total>"`, or `None` on overflow.
pub fn report_line(name: &str, qty: u32, unit_cents: u64) -> Option<String> {
    todo!("implement report_line (see README, step 7)")
}

/// Joins one report line per row; any overflow fails the whole report.
pub fn report(rows: &[(&str, u32, u64)]) -> Option<String> {
    todo!("implement report (see README, step 7)")
}
