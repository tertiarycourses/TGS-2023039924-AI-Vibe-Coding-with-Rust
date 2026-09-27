use activity01::{line_total, MAX_QTY};

#[test]
fn normal_line_total() {
    assert_eq!(line_total(250, 4), Some(1_000));
}

#[test]
fn zero_quantity_is_rejected() {
    assert_eq!(line_total(250, 0), None);
}

#[test]
fn maximum_quantity_is_accepted() {
    assert_eq!(line_total(1, MAX_QTY), Some(10_000));
}

#[test]
fn quantity_above_maximum_is_rejected() {
    assert_eq!(line_total(1, MAX_QTY + 1), None);
}

#[test]
fn overflow_is_rejected_not_wrapped() {
    assert_eq!(line_total(u64::MAX, 2), None);
}
