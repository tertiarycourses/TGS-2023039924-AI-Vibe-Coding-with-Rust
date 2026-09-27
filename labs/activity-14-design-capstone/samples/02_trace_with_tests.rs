// Sample: tests named after requirements form a living trace matrix.
/// R1: point = usage x lead + safety. R2: above the point, no order.
/// R3: order up to point + 7 days of usage. R4: overflow gives None.
pub fn suggest(
    on_hand: u32,
    usage: u32,
    lead: u32,
    safety: u32,
) -> Option<u32> {
    let point = usage.checked_mul(lead)?.checked_add(safety)?;
    if on_hand > point {
        return None;
    }
    point
        .checked_add(usage.checked_mul(7)?)?
        .checked_sub(on_hand)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn r1_r3_below_point_orders_up_to_target() {
        assert_eq!(suggest(50, 10, 5, 20), Some(90));
    }

    #[test]
    fn r2_above_point_no_order() {
        assert_eq!(suggest(71, 10, 5, 20), None);
    }

    #[test]
    fn r4_overflow_is_none() {
        assert_eq!(suggest(0, u32::MAX, 2, 0), None);
    }
}
