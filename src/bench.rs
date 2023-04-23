#![feature(test)]

extern crate test;

#[cfg(test)]
mod benchmarks {
    use arbitrary_int::*;
    use num_bigint::BigUint;

    fn product_range<T: core::ops::Mul<Output = T> + From<u64>>(lo: u64, hi: u64) -> T {
        if lo.abs_diff(hi) < 16 {
            let mut res = T::from(1);
            for e in lo..hi {
                res = res * T::from(e);
            }
            res
        } else {
            product_range::<T>(lo, lo + (hi - lo) / 2) * product_range(lo + (hi - lo) / 2, hi)
        }
    }

    fn compute_factorial<T: core::ops::Mul<Output = T> + From<u64>>(n: u64) -> T {

        // if n < 2 {
        //     return T::from(1);
        // }

        // let mut res = T::from(1);
        // for e in 1..=n {
        //     res = res * T::from(e);
        // }
        // res

        if n < 2 {
            T::from(1)
        } else {
            product_range(2, n + 1)
        }
    }

    fn do_small_addition<T: core::ops::Add + From<u64>>(b: &mut test::Bencher) {
        let mut state = 463022728790906309_u64;
        let mut myrand = || {
            state = state.wrapping_add(750667081032403741);
            state
        };
        b.iter(|| {
            for _ in 0..(1 << 20) {
                test::black_box(T::from(myrand() % 16) + T::from(myrand() % 16));
            }
        });
    }

    fn do_small_multiplication<T: core::ops::Mul + From<u64>>(b: &mut test::Bencher) {
        let mut state = 463022728790906309_u64;
        let mut myrand = || {
            state = state.wrapping_add(750667081032403741);
            state
        };
        b.iter(|| {
            for _ in 0..(1 << 20) {
                test::black_box(T::from(myrand() % 16) * T::from(myrand() % 16));
            }
        });
    }

    fn do_small_factorial<T: core::ops::Mul<Output = T> + From<u64>>(b: &mut test::Bencher) {
        let mut state = 463022728790906309_u64;
        let mut myrand = || {
            state = state.wrapping_add(750667081032403741);
            state
        };
        b.iter(|| {
            for _ in 0..(1 << 10) {
                test::black_box(compute_factorial::<T>(myrand() % 8));
            }
        });
    }

    fn do_medium_factorial<T: core::ops::Mul<Output = T> + From<u64>>(b: &mut test::Bencher) {
        let mut state = 463022728790906309_u64;
        let mut myrand = || {
            state = state.wrapping_add(750667081032403741);
            state
        };
        b.iter(|| {
            for _ in 0..(1 << 10) {
                test::black_box(compute_factorial::<T>(myrand() % 21));
            }
        });
    }

    #[bench]
    fn adding_small_native(b: &mut test::Bencher) {
        do_small_addition::<u64>(b);
    }

    #[bench]
    fn adding_small(b: &mut test::Bencher) {
        assert_eq!(1, compute_factorial::<u64>(0));
        assert_eq!(1, compute_factorial::<u64>(1));
        assert_eq!(2, compute_factorial::<u64>(2));
        assert_eq!(6, compute_factorial::<u64>(3));
        do_small_addition::<CHANGEME>(b);
    }

    #[bench]
    fn adding_small_biguint(b: &mut test::Bencher) {
        do_small_addition::<BigUint>(b);
    }

    #[bench]
    fn multiplying_small_native(b: &mut test::Bencher) {
        do_small_multiplication::<u64>(b);
    }

    #[bench]
    fn multiplying_small(b: &mut test::Bencher) {
        do_small_multiplication::<CHANGEME>(b);
    }

    #[bench]
    fn multiplying_small_biguint(b: &mut test::Bencher) {
        do_small_multiplication::<BigUint>(b);
    }

    #[bench]
    fn factorial_small(b: &mut test::Bencher) {
        do_small_factorial::<CHANGEME>(b);
    }

    #[bench]
    fn factorial_small_biguint(b: &mut test::Bencher) {
        do_small_factorial::<BigUint>(b);
    }

    #[bench]
    fn factorial_small_native(b: &mut test::Bencher) {
        do_small_factorial::<u64>(b);
    }

    #[bench]
    fn factorial_medium(b: &mut test::Bencher) {
        do_medium_factorial::<CHANGEME>(b);
    }

    #[bench]
    fn factorial_medium_biguint(b: &mut test::Bencher) {
        do_medium_factorial::<BigUint>(b);
    }

    #[bench]
    fn factorial_medium_native(b: &mut test::Bencher) {
        do_medium_factorial::<u64>(b);
    }
}
