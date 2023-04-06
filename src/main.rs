fn main() {}

#[cfg(test)]
mod tests {
    use arbitrary_int::*;
    use num_bigint::BigUint;

    #[test]
    fn test_small_add() {
        assert_eq!(
            CHANGEME::from(1_u32) + CHANGEME::from(2_u32),
            CHANGEME::from(3_u32)
        );
    }

    #[test]
    fn test_medium_add() {
        assert_eq!(
            CHANGEME::from(u64::MAX) + CHANGEME::from(u64::MAX),
            CHANGEME::from(u64::MAX as u128 * 2)
        );
    }

    #[test]
    fn test_large_add() {
        assert_eq!(
            CHANGEME::from(u128::MAX) + CHANGEME::from(u128::MAX),
            CHANGEME::from(BigUint::from(u128::MAX) * BigUint::from(2_u32))
        );
    }
}
