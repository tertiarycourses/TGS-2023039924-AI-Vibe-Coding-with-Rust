use activity04::{parse_command, Command};

#[test]
fn parses_list() {
    assert_eq!(parse_command(&["list"]), Ok(Command::List));
}

#[test]
fn parses_add_and_remove() {
    assert_eq!(
        parse_command(&["add", "ABC-0001", "5"]),
        Ok(Command::Add {
            sku: "ABC-0001".into(),
            qty: 5
        })
    );
    assert_eq!(
        parse_command(&["remove", "ABC-0001", "2"]),
        Ok(Command::Remove {
            sku: "ABC-0001".into(),
            qty: 2
        })
    );
}

#[test]
fn rejects_zero_quantity() {
    assert_eq!(
        parse_command(&["add", "ABC-0001", "0"]),
        Err("quantity must be at least 1".to_string())
    );
}

#[test]
fn rejects_non_numeric_and_negative_quantities() {
    assert!(parse_command(&["add", "ABC-0001", "five"]).is_err());
    assert!(parse_command(&["remove", "ABC-0001", "-3"]).is_err());
}

#[test]
fn rejects_unknown_verbs_missing_and_extra_words() {
    assert!(parse_command(&["delete", "ABC-0001", "1"]).is_err());
    assert!(parse_command(&["add", "ABC-0001"]).is_err());
    assert!(parse_command(&["list", "now"]).is_err());
    assert!(parse_command(&[]).is_err());
}
