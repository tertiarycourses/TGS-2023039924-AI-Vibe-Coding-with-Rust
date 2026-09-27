use activity07::{apply, apply_all, Movement, StockError};

#[test]
fn receive_and_ship() {
    assert_eq!(apply(10, Movement::Receive(5)), Ok(15));
    assert_eq!(apply(10, Movement::Ship(10)), Ok(0));
}

#[test]
fn shipping_too_much_is_insufficient() {
    assert_eq!(
        apply(3, Movement::Ship(5)),
        Err(StockError::Insufficient {
            available: 3,
            requested: 5
        })
    );
}

#[test]
fn adjustments_are_bounded_both_ways() {
    assert_eq!(apply(10, Movement::Adjust(-4)), Ok(6));
    assert_eq!(
        apply(3, Movement::Adjust(-4)),
        Err(StockError::NegativeResult)
    );
    assert_eq!(
        apply(u32::MAX, Movement::Adjust(1)),
        Err(StockError::Overflow)
    );
    assert_eq!(
        apply(1, Movement::Adjust(i64::MAX)),
        Err(StockError::Overflow)
    );
}

#[test]
fn receive_overflow_is_reported() {
    assert_eq!(
        apply(u32::MAX, Movement::Receive(1)),
        Err(StockError::Overflow)
    );
}

#[test]
fn apply_all_runs_in_order() {
    let moves = [
        Movement::Receive(10),
        Movement::Ship(4),
        Movement::Adjust(-1),
    ];
    assert_eq!(apply_all(0, &moves), Ok(5));
}

#[test]
fn apply_all_stops_at_the_first_error() {
    let moves = [
        Movement::Receive(2),
        Movement::Ship(5),
        Movement::Receive(100),
    ];
    assert_eq!(
        apply_all(0, &moves),
        Err(StockError::Insufficient {
            available: 2,
            requested: 5
        })
    );
}
