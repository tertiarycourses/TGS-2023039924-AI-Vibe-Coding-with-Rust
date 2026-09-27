use activity10::{parse_all, parse_record, ParseError, Record};

const SAMPLE: &str = include_str!("../../data/stock.csv");

#[test]
fn parses_a_valid_line_with_spaces() {
    assert_eq!(
        parse_record(" ABC-0001 , 40 , 899 "),
        Ok(Record {
            sku: "ABC-0001".into(),
            qty: 40,
            unit_cents: 899
        })
    );
}

#[test]
fn reports_each_error_kind() {
    assert_eq!(parse_record("ABC-0001,4"), Err(ParseError::FieldCount(2)));
    assert_eq!(parse_record(",5,100"), Err(ParseError::EmptySku));
    assert_eq!(
        parse_record("ABC-0002,twelve,2499"),
        Err(ParseError::BadNumber {
            field: "qty",
            value: "twelve".into()
        })
    );
    assert!(matches!(
        parse_record("ABC-0002,-1,2499"),
        Err(ParseError::BadNumber { field: "qty", .. })
    ));
}

#[test]
fn error_messages_are_readable() {
    assert_eq!(
        ParseError::FieldCount(2).to_string(),
        "expected 3 fields, found 2"
    );
    let bad = ParseError::BadNumber {
        field: "qty",
        value: "twelve".into(),
    };
    assert_eq!(bad.to_string(), "qty is not a whole number: \"twelve\"");
}

#[test]
fn parse_all_keeps_good_lines_and_numbers_bad_ones() {
    let (records, errors) = parse_all(SAMPLE);
    let skus: Vec<&str> = records.iter().map(|r| r.sku.as_str()).collect();
    assert_eq!(skus, ["ABC-0001", "ABC-0003"]);
    let lines: Vec<usize> = errors.iter().map(|(line, _)| *line).collect();
    assert_eq!(lines, [3, 4, 6]);
}

#[test]
fn question_mark_works_with_boxed_errors(
) -> Result<(), Box<dyn std::error::Error>> {
    let record = parse_record("ABC-0005,7,350")?;
    assert_eq!(record.qty, 7);
    Ok(())
}
