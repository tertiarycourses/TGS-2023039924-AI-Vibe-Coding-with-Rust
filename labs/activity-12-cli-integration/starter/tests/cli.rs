use std::process::Command;

const DATA: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../data/stock.csv");
const REPORT: &str = "ITEMS 4\nTOTAL $526.55\nLOW STOCK ABC-0003";

fn stockpilot() -> Command {
    Command::new(env!("CARGO_BIN_EXE_activity12"))
}

#[test]
fn library_report_matches_the_specification() {
    assert_eq!(
        activity12::run(include_str!("../../data/stock.csv"), 5),
        Ok(REPORT.to_string())
    );
}

#[test]
fn empty_input_reports_zero() {
    assert_eq!(
        activity12::run("", 5),
        Ok("ITEMS 0\nTOTAL $0.00\nLOW STOCK none".to_string())
    );
}

#[test]
fn malformed_row_is_an_error() {
    assert_eq!(
        activity12::run("ABC-0001,1", 5),
        Err("line 1: expected 3 fields".to_string())
    );
}

#[test]
fn binary_prints_the_report_and_exits_zero() {
    let output = stockpilot().arg(DATA).output().expect("binary runs");
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim_end(), REPORT);
}

#[test]
fn binary_accepts_a_threshold() {
    let output = stockpilot()
        .args([DATA, "13"])
        .output()
        .expect("binary runs");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("LOW STOCK ABC-0002, ABC-0003, ABC-0004"));
}

#[test]
fn usage_error_exits_two() {
    let output = stockpilot().output().expect("binary runs");
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("usage"));
}

#[test]
fn unreadable_file_exits_one() {
    let output = stockpilot()
        .arg("no-such-file.csv")
        .output()
        .expect("binary runs");
    assert_eq!(output.status.code(), Some(1));
}
