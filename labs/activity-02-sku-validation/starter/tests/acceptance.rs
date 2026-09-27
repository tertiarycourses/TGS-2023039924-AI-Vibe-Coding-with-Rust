use activity02::{is_valid_sku, normalise_sku};

#[test]
fn accepts_the_documented_format() {
    assert!(is_valid_sku("ABC-1234"));
}

#[test]
fn trims_surrounding_whitespace() {
    assert!(is_valid_sku("  ABC-1234\n"));
}

#[test]
fn rejects_lower_case_letters() {
    assert!(!is_valid_sku("abc-1234"));
}

#[test]
fn rejects_wrong_layout_and_letters_in_digits() {
    assert!(!is_valid_sku("AB-12345"));
    assert!(!is_valid_sku("ABC-12A4"));
    assert!(!is_valid_sku(""));
}

#[test]
fn rejects_non_ascii_letters() {
    assert!(!is_valid_sku("ÄBC-1234"));
}

#[test]
fn normalises_valid_input_to_upper_case() {
    assert_eq!(normalise_sku(" abc-1234 "), Some("ABC-1234".to_string()));
    assert_eq!(normalise_sku("bad"), None);
}
