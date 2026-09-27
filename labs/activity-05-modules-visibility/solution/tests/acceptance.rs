use activity05::money::format_cents;
use activity05::report::{report, report_line};

#[test]
fn formats_cents_with_two_decimal_places() {
    assert_eq!(format_cents(0), "$0.00");
    assert_eq!(format_cents(5), "$0.05");
    assert_eq!(format_cents(1205), "$12.05");
    assert_eq!(format_cents(100_000), "$1000.00");
}

#[test]
fn builds_one_report_line() {
    assert_eq!(
        report_line("Cable", 3, 250),
        Some("Cable x3 @ $2.50 = $7.50".to_string())
    );
}

#[test]
fn joins_lines_in_order() {
    let rows = [("Cable", 3, 250), ("Charger", 1, 2499)];
    assert_eq!(
        report(&rows),
        Some(
            "Cable x3 @ $2.50 = $7.50\nCharger x1 @ $24.99 = $24.99"
                .to_string()
        )
    );
}

#[test]
fn overflow_fails_the_whole_report() {
    assert_eq!(report_line("Bulk", 2, u64::MAX), None);
    assert_eq!(report(&[("Cable", 1, 1), ("Bulk", 2, u64::MAX)]), None);
}
