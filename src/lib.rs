use num_bigint::BigUint;
use std::ops::Add;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum CHANGEME {
    Small(u64),
    Medium(u128),
    Large(BigUint),
}
impl Default for CHANGEME {
    fn default() -> Self {
        CHANGEME::Small(0)
    }
}

impl From<u8> for CHANGEME {
    fn from(value: u8) -> Self {
        CHANGEME::Small(value as u64)
    }
}
impl From<u16> for CHANGEME {
    fn from(value: u16) -> Self {
        CHANGEME::Small(value as u64)
    }
}
impl From<u32> for CHANGEME {
    fn from(value: u32) -> Self {
        CHANGEME::Small(value as u64)
    }
}
impl From<u64> for CHANGEME {
    fn from(value: u64) -> Self {
        CHANGEME::Small(value)
    }
}
impl From<u128> for CHANGEME {
    fn from(value: u128) -> Self {
        CHANGEME::Medium(value)
    }
}
impl From<BigUint> for CHANGEME {
    fn from(value: BigUint) -> Self {
        CHANGEME::Large(value)
    }
}
impl From<&BigUint> for CHANGEME {
    fn from(value: &BigUint) -> Self {
        CHANGEME::Large(value.clone())
    }
}

pub mod util {
    use likely_stable::*;
    use num_bigint::BigUint;

    use crate::CHANGEME;

    pub fn add_small(a: u64, b: u64) -> CHANGEME {
        let (sum, carry) = a.overflowing_add(b);
        if likely(!carry) {
            CHANGEME::Small(sum)
        } else {
            CHANGEME::Medium(((carry as u128) << 64) + sum as u128)
        }
    }

    pub fn add_medium(a: u128, b: u128) -> CHANGEME {
        let (sum, carry) = a.overflowing_add(b);
        if likely(!carry) {
            CHANGEME::Medium(sum)
        } else {
            CHANGEME::Large((BigUint::from(carry as u64) << 128) + sum)
        }
    }

    pub fn add_large_to_medium(a: &BigUint, b: u128) -> CHANGEME {
        let mut res = BigUint::from(b);
        res += a;
        CHANGEME::Large(res)
    }

    pub fn add_large(a: &BigUint, b: &BigUint) -> CHANGEME {
        CHANGEME::Large(a + b)
    }
}

impl CHANGEME {
    fn compute_sum(&self, other: &CHANGEME) -> CHANGEME {
        use CHANGEME::*;
        match (self, other) {
            (Small(a), Small(b)) => util::add_small(*a, *b),
            (Small(a), Medium(b)) => util::add_medium(*a as u128, *b),
            (Medium(a), Small(b)) => util::add_medium(*a, *b as u128),
            (Medium(a), Medium(b)) => util::add_medium(*a, *b),
            (Medium(a), Large(b)) => util::add_large_to_medium(b, *a),
            (Large(a), Medium(b)) => util::add_large_to_medium(a, *b),
            (Large(a), Large(b)) => util::add_large(a, b),
            _ => Small(0),
        }
    }
}

impl Add<&CHANGEME> for &CHANGEME {
    type Output = CHANGEME;
    fn add(self, rhs: &CHANGEME) -> Self::Output {
        self.compute_sum(rhs)
    }
}

impl Add<CHANGEME> for CHANGEME {
    type Output = CHANGEME;
    fn add(self, rhs: CHANGEME) -> Self::Output {
        self.compute_sum(&rhs)
    }
}
