pub fn num_len(x: u64) -> u8 {
    let mut l: u8 = 1;
    while x >= 10u64.pow(l as u32) {
        l += 1;
    }

    l
}

pub fn prepend_digit(num: u64, digit: u8) -> u64 {
    let num_len = num_len(num);

    let pow = 10u64.pow((num_len) as u32);

    (digit as u64) * pow + num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepend_digit() {
        assert_eq!(prepend_digit(123, 9), 9123);
        assert_eq!(prepend_digit(0, 1), 10);
        assert_eq!(prepend_digit(5, 1), 15);
        assert_eq!(prepend_digit(1000, 5), 51000);
        assert_eq!(prepend_digit(98765, 1), 198765);
    }

    #[test]
    fn test_num_len() {
        assert_eq!(num_len(123), 3);
        assert_eq!(num_len(0), 1);
        assert_eq!(num_len(1), 1);
        assert_eq!(num_len(9999), 4);
    }
}