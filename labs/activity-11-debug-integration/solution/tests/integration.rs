use activity11::{low_stock, parse_lines, summarise, total_cents, Summary};

const SAMPLE: &str = include_str!("../../data/stock.csv");

#[test]
fn parses_every_stock_line() {
    assert_eq!(parse_lines(SAMPLE).unwrap().len(), 4);
}

#[test]
fn total_includes_the_first_line() {
    let lines = parse_lines(SAMPLE).unwrap();
    assert_eq!(total_cents(&lines), Some(52_655));
}

#[test]
fn low_stock_is_strictly_below_threshold() {
    let lines = parse_lines(SAMPLE).unwrap();
    assert_eq!(low_stock(&lines, 5), ["ABC-0003"]);
}

#[test]
fn end_to_end_summary() {
    assert_eq!(
        summarise(SAMPLE, 5),
        Ok(Summary {
            lines: 4,
            total_cents: 52_655,
            low_stock: vec!["ABC-0003".to_string()],
        })
    );
}

#[test]
fn malformed_line_reports_its_number() {
    assert_eq!(
        parse_lines("sku,qty,unit_cents\nABC-0001,4"),
        Err("line 2: expected 3 fields".to_string())
    );
}

#[test]
fn overflow_is_an_error_not_a_panic() {
    assert_eq!(
        summarise("ABC-0001,2,18446744073709551615", 0),
        Err("stock value overflowed u64".to_string())
    );
}
