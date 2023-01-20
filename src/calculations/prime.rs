use num_integer::Roots;

pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..(n.sqrt() + 1) {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_valid_prime_numbers() {
        assert!(is_prime(2));
        assert!(is_prime(17));
        assert!(is_prime(79));
        assert!(is_prime(97));
        assert!(is_prime(389));
        assert!(is_prime(5331637));
    }

    #[test]
    fn check_invalid_prime_numbers() {
        assert_eq!(false, is_prime(4));
        assert_eq!(false, is_prime(12));
        assert_eq!(false, is_prime(35));
        assert_eq!(false, is_prime(100));
        assert_eq!(false, is_prime(410));
        assert_eq!(false, is_prime(5331638));
    }
}
