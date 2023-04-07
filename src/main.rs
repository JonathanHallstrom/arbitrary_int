fn main() {}

#[cfg(test)]
mod tests {
    use arbitrary_int::*;
    use num_bigint::BigUint;

    #[test]
    fn test_large_conversion() {
        assert_eq!(
            CHANGEME::from(u128::MAX),
            CHANGEME::from(BigUint::from(u128::MAX))
        );
        assert_eq!(
            CHANGEME::from(u64::MAX),
            CHANGEME::from(BigUint::from(u64::MAX))
        );
    }

    #[test]
    fn test_small_add() {
        for i in (0..u64::MAX).step_by(2_906_698_589_538_647) {
            for j in (0..u64::MAX).step_by(9_314_729_677_867_609) {
                assert_eq!(
                    CHANGEME::from(i) + CHANGEME::from(j),
                    CHANGEME::from(i as u128 + j as u128)
                );
            }
        }
    }

    #[test]
    fn test_medium_add() {
        {
            // step by doesnt work for value > usize::MAX so I made my own for loops
            let mut i = 0;
            while i < u128::MAX - 9355922656428016303878336798081429281 {
                i += 9355922656428016303878336798081429281;
                let mut j = 0;
                while j < u128::MAX - 3162064039811835189013542041112339911 {
                    j += 3162064039811835189013542041112339911;
                    assert_eq!(
                        CHANGEME::from(i) + CHANGEME::from(j),
                        CHANGEME::from(BigUint::from(i) + BigUint::from(j))
                    );
                }
            }
        }
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

    #[test]
    fn test_small_mul() {
        for i in (0..u64::MAX).step_by(2_906_698_589_538_647) {
            for j in (0..u64::MAX).step_by(9_314_729_677_867_609) {
                assert_eq!(
                    CHANGEME::from(i) * CHANGEME::from(j),
                    CHANGEME::from(i as u128 * j as u128)
                );
            }
        }
    }

    #[test]
    fn test_medium_mul() {
        {
            // step by doesnt work for value > usize::MAX so I made my own for loops
            let mut i = 0;
            while i < u128::MAX - 9355922656428016303878336798081429281 {
                i += 9355922656428016303878336798081429281;
                let mut j = 0;
                while j < u128::MAX - 3162064039811835189013542041112339911 {
                    j += 3162064039811835189013542041112339911;
                    assert_eq!(
                        CHANGEME::from(i) * CHANGEME::from(j),
                        CHANGEME::from(BigUint::from(i) * BigUint::from(j))
                    );
                }
            }
        }
        assert_eq!(
            CHANGEME::from(u64::MAX) * CHANGEME::from(u64::MAX),
            CHANGEME::from((u64::MAX as u128).pow(2))
        );
    }

    #[test]
    fn test_large_mul() {
        assert_eq!(
            CHANGEME::from(u128::MAX) * CHANGEME::from(u128::MAX),
            CHANGEME::from(BigUint::from(u128::MAX).pow(2))
        );
    }
}
