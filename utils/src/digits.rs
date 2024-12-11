// Manipulate digits in an integer without first converting to string
pub fn count_digits(mut num: usize) -> u8 {
    let mut result = 0;
    loop {
        num /= 10;
        result += 1;
        if num == 0 {
            break;
        }
    }
    result
}

// e.g. split_by_digit(1024, 2) => (10, 24)
pub fn split_by_digit(num: usize, digit: u8) -> (usize, usize) {
    let divider = 10usize.pow(digit as u32);
    (num / divider, num % divider)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_digits() {
        assert_eq!(count_digits(0), 1);
        assert_eq!(count_digits(1), 1);
        assert_eq!(count_digits(2), 1);
        assert_eq!(count_digits(10), 2);
        assert_eq!(count_digits(11), 2);
        assert_eq!(count_digits(42), 2);
        assert_eq!(count_digits(99), 2);
        assert_eq!(count_digits(100), 3);
    }
}
