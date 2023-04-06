use likely_stable::{likely, unlikely};
use num_bigint::BigUint;
use std::ops::Add;

use crate::util::add_tiny;

pub mod util {
    pub fn add_tiny(a: u64, b: u64) -> (u64, u64) {
        let (sum, carry) = a.overflowing_add(b);
        (sum, carry as u64)
    }
    pub fn add_small(a: u128, b: u128) -> (u128, u128) {
        let (sum, carry) = a.overflowing_add(b);
        (sum, carry as u128)
    }
}

pub enum CHANGEME {
    Tiny(u64),
    Small(u128),
    Big(BigUint),
}

impl CHANGEME {
    fn compute_sum(&self, other: &CHANGEME) -> CHANGEME {
        use CHANGEME::*;
        match (self, other) {
            (Tiny(a), Tiny(b)) => {
                let (sum, carry) = add_tiny(*a, *b);
                if likely(carry == 0) {
                    Tiny(sum)
                } else {
                    Small((sum as u128) << 64 + carry as u128)
                }
            }
            _ => Tiny(0),
        }
    }
}

impl Add<&CHANGEME> for &CHANGEME {
    type Output = CHANGEME;
    fn add(self, rhs: &CHANGEME) -> Self::Output {
        self.compute_sum(rhs)
    }
}
