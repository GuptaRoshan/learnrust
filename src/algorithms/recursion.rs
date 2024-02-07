
pub fn factorial(n: u64) -> u64 {
    if n == 0 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_zero() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn test_factorial_positive() {
        assert_eq!(factorial(5), 120);
    }
}