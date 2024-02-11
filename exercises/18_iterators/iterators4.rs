// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

use std::iter::successors;

pub fn factorial(num: u64) -> u64 {
    successors(Some((1 as u64, 1 as u64)), |(f, i)| Some((f * i, i + 1)))
        .take(num as usize + 1)
        .map(|(f, v)| f)
        .last()
        .unwrap_or(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
