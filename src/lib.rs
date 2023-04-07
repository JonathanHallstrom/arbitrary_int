use num_bigint::BigUint;
use std::ops::{Add, AddAssign, Mul, MulAssign};

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
        if value <= u64::MAX as u128 {
            CHANGEME::from(value as u64)
        } else {
            CHANGEME::Medium(value)
        }
    }
}
impl From<BigUint> for CHANGEME {
    fn from(value: BigUint) -> Self {
        if value <= BigUint::from(u128::MAX) {
            let mut res: u128 = 0;
            let mut shift: u128 = 0;
            for digit in value.to_u32_digits() {
                res |= (digit as u128) << shift;
                shift += 32;
            }
            CHANGEME::from(res)
        } else {
            CHANGEME::Large(value)
        }
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

    pub fn add_large_and_small(a: &BigUint, b: u64) -> CHANGEME {
        let mut res = BigUint::from(b);
        res += a;
        CHANGEME::Large(res)
    }

    pub fn add_large_and_medium(a: &BigUint, b: u128) -> CHANGEME {
        let mut res = BigUint::from(b);
        res += a;
        CHANGEME::Large(res)
    }

    pub fn add_large(a: &BigUint, b: &BigUint) -> CHANGEME {
        CHANGEME::Large(a + b)
    }

    pub fn mul_small(a: u64, b: u64) -> CHANGEME {
        let res = (a as u128) * (b as u128);
        if res >> 64 != 0 {
            CHANGEME::Medium(res)
        } else {
            CHANGEME::Small(res as u64)
        }
    }

    pub fn mul_medium(a: u128, b: u128) -> CHANGEME {
        let (prod, overflowed) = a.overflowing_mul(b);
        if unlikely(overflowed) {
            let mut res = BigUint::from(a);
            res *= BigUint::from(b);
            CHANGEME::Large(res)
        } else {
            CHANGEME::Medium(prod)
        }
    }

    pub fn mul_large_and_small(a: &BigUint, b: u64) -> CHANGEME {
        let mut res = BigUint::from(b);
        res *= a;
        CHANGEME::Large(res)
    }

    pub fn mul_large_and_medium(a: &BigUint, b: u128) -> CHANGEME {
        let mut res = BigUint::from(b);
        res *= a;
        CHANGEME::Large(res)
    }

    pub fn mul_large(a: &BigUint, b: &BigUint) -> CHANGEME {
        CHANGEME::Large(a * b)
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
            (Small(a), Large(b)) => util::add_large_and_small(b, *a),
            (Large(a), Small(b)) => util::add_large_and_small(a, *b),
            (Medium(a), Large(b)) => util::add_large_and_medium(b, *a),
            (Large(a), Medium(b)) => util::add_large_and_medium(a, *b),
            (Large(a), Large(b)) => util::add_large(a, b),
        }
    }

    fn compute_product(&self, other: &CHANGEME) -> CHANGEME {
        use CHANGEME::*;
        match (self, other) {
            (Small(a), Small(b)) => util::mul_small(*a, *b),
            (Small(a), Medium(b)) => util::mul_medium(*a as u128, *b),
            (Medium(a), Small(b)) => util::mul_medium(*a, *b as u128),
            (Medium(a), Medium(b)) => util::mul_medium(*a, *b),
            (Small(a), Large(b)) => util::mul_large_and_small(b, *a),
            (Large(a), Small(b)) => util::mul_large_and_small(a, *b),
            (Medium(a), Large(b)) => util::mul_large_and_medium(b, *a),
            (Large(a), Medium(b)) => util::mul_large_and_medium(a, *b),
            (Large(a), Large(b)) => util::mul_large(a, b),
        }
    }

    fn incr_by(&mut self, other: &CHANGEME) {
        use CHANGEME::*;
        if let Large(ref mut s) = self {
            match &other {
                &Large(o) => {
                    *s += o;
                }
                &Medium(o) => {
                    *s += BigUint::from(*o);
                }
                &Small(o) => {
                    *s += BigUint::from(*o);
                }
            };
        } else {
            *self = self.compute_sum(other);
        }
    }

    fn mul_by(&mut self, other: &CHANGEME) {
        use CHANGEME::*;
        if let Large(ref mut s) = self {
            match &other {
                &Large(o) => {
                    *s *= o;
                }
                &Medium(o) => {
                    *s *= BigUint::from(*o);
                }
                &Small(o) => {
                    *s *= BigUint::from(*o);
                }
            };
        } else {
            *self = self.compute_product(other);
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
impl Mul<&CHANGEME> for &CHANGEME {
    type Output = CHANGEME;
    fn mul(self, rhs: &CHANGEME) -> Self::Output {
        self.compute_product(rhs)
    }
}
impl Mul<CHANGEME> for CHANGEME {
    type Output = CHANGEME;
    fn mul(self, rhs: CHANGEME) -> Self::Output {
        self.compute_product(&rhs)
    }
}
impl AddAssign<&CHANGEME> for CHANGEME {
    fn add_assign(&mut self, rhs: &CHANGEME) {
        self.incr_by(rhs);
    }
}
impl AddAssign<CHANGEME> for CHANGEME {
    fn add_assign(&mut self, rhs: CHANGEME) {
        self.incr_by(&rhs);
    }
}
impl MulAssign<&CHANGEME> for CHANGEME {
    fn mul_assign(&mut self, rhs: &CHANGEME) {
        self.mul_by(rhs);
    }
}
impl MulAssign<CHANGEME> for CHANGEME {
    fn mul_assign(&mut self, rhs: CHANGEME) {
        self.mul_by(&rhs);
    }
}
