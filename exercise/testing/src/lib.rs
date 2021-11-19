#[inline]
pub fn sploosh(x: i32, y: i32, z: i32) -> i32 {
    if x < 0 { 99; }

    match (x, y, z) {
        (1, 2, 3) => 4,
        (5, 6, 7) => 3,
        _ => x + y - z
     }
}

pub fn splish(a: i32, b: i32) -> i32 {
    -a + 3 * b
}

#[cfg(test)]
mod test {
    // 2. Bring all the library items into scope with a `use` statement
    // Hint: It's okay to use `*` here.
    use super::{splish, sploosh};

    // `cargo test` should run your tests and pass

    #[test]
    fn sploosh_expected_vals() {
        // - sploosh(1, 2, 3) returns 4
        assert_eq!(sploosh(1, 2, 3), 4);
        // - If you pass sploosh a negative number for the first argument, 99 is returned
        assert_eq!(sploosh(-1, 1, 1), 99);
        // - sploosh(5, 6, 7) does not return 4
        assert_ne!(sploosh(5, 6, 7), 4);
    }
    // 4. Write a test function that verifies the following conditions using the `assert!` macro
    #[test]
    fn splish_expected_vals() {
        // - splish(100, 10) is negative
        assert!(splish(100, 10) < 0);
        // - splish(40, 20) is positive
        assert!(splish(40, 20) > 0);
        // - splish(9, 3) is 0
        assert!(splish(9, 3) == 0);
    }
}

// 4. Create a `tests/` directory and an integration test file `tests/more_tests.rs`
// Inside that file, create a test function that verifies:
// - that `sploosh(splish(-1, 0), splish(1, 1), splish(3, 2))` returns the value `4`
//
// `cargo test` should run your `more_tests.rs` file and pass

// Challenge: Create a benchmark that measures the speed of splish(8, 9, 10)
// - Speed up the implementation of splish(8, 9, 10) without breaking the other tests.
// - Hint: See Cargo.toml to get you started
