// Sample: unit tests in one file. Run with the --test flag (see README).
pub fn low_stock(levels: &[(&str, u32)], threshold: u32) -> Vec<String> {
    let mut skus: Vec<String> = levels
        .iter()
        .filter(|(_, qty)| *qty < threshold)
        .map(|(sku, _)| sku.to_string())
        .collect();
    skus.sort();
    skus
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strictly_below_threshold() {
        assert_eq!(low_stock(&[("A", 4), ("B", 5)], 5), ["A"]);
    }

    #[test]
    fn sorted_output() {
        assert_eq!(low_stock(&[("B", 0), ("A", 1)], 5), ["A", "B"]);
    }

    #[test]
    fn nothing_is_low_at_zero() {
        assert!(low_stock(&[("A", 0)], 0).is_empty());
    }
}
