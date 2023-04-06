// use num_bigint::{BigInt, Sign};

fn main() {

}

#[cfg(test)]
mod tests {
    use arbitrary_int::*;
    
    #[test]
    fn small_test() {
        assert_eq!(util::add_tiny(1, 1), (2, 0));
    }

    #[test]
    fn big_test() {
        assert_eq!(util::add_tiny(u64::MAX, u64::MAX), (u64::MAX - 1, 1));
    }
}