use activity13::{needs_reorder, reorder_point};

#[test]
fn r1_formula() {
    assert_eq!(reorder_point(12, 5, 20), Some(80));
    assert_eq!(reorder_point(0, 5, 20), Some(20));
}

#[test]
fn r2_overflow_is_reported() {
    assert_eq!(reorder_point(u32::MAX, 2, 0), None);
    assert_eq!(reorder_point(1, u32::MAX, 1), None);
}

#[test]
fn r3_boundary() {
    assert!(needs_reorder(79, 80));
    assert!(needs_reorder(80, 80));
    assert!(!needs_reorder(81, 80));
}
