static MAX_RANGE: usize = 52;

pub struct FibonacciCache {
    index: u8,
    number: u64,
    cache: Vec<u64>,
}

pub fn is_in_fibonacci_sequence(
    number: u64,
    fib_cache: Option<FibonacciCache>,
) -> Result<bool, String> {
    let mut fib_cache = match fib_cache {
        Some(cache) => cache,
        None => FibonacciCache {
            index: 0,
            number: 0,
            cache: vec![0; MAX_RANGE + 1], // initializing memory
        },
    };
    if fib_cache.index >= MAX_RANGE as u8 {
        let error_message = format!("{} is out of bound of the fibonacci sequence", number);
        return Err(error_message);
    };

    match number {
        number if fib_cache.number > number => return Ok(false),
        number if fib_cache.number == number => return Ok(true),
        _ => {
            let new_fib_index = fib_cache.index + 1;
            let fib_result =
                fibonacci_sequence(Some(fib_cache.number), new_fib_index, &fib_cache.cache);
            fib_cache.cache[new_fib_index as usize] = fib_result;
            return is_in_fibonacci_sequence(
                number,
                Some(FibonacciCache {
                    index: new_fib_index,
                    number: fib_result,
                    cache: fib_cache.cache,
                }),
            );
        }
    }

    fn fibonacci_sequence(last_number: Option<u64>, index: u8, fib_cache: &Vec<u64>) -> u64 {
        if index <= 1 {
            return 1;
        }

        if fib_cache[index as usize] != 0 {
            let cached_value = fib_cache[index as usize];
            return cached_value;
        }
        match last_number {
            Some(n) => return n + fibonacci_sequence(None, index - 2, fib_cache),

            None => {
                return fibonacci_sequence(None, index - 1, fib_cache)
                    + fibonacci_sequence(None, index - 2, fib_cache)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_fibonacci_numbers() {
        assert!(is_in_fibonacci_sequence(0, None).unwrap());
        assert!(is_in_fibonacci_sequence(1, None).unwrap());
        assert!(is_in_fibonacci_sequence(5, None).unwrap());
        assert!(is_in_fibonacci_sequence(377, None).unwrap());
        assert!(is_in_fibonacci_sequence(102334155, None).unwrap());
        assert!(is_in_fibonacci_sequence(32951280099, None).unwrap());
    }
    #[test]
    fn invalid_fibonacci_numbers() {
        assert_eq!(false, is_in_fibonacci_sequence(4, None).unwrap());
        assert_eq!(false, is_in_fibonacci_sequence(6, None).unwrap());
        assert_eq!(false, is_in_fibonacci_sequence(9, None).unwrap());
        assert_eq!(false, is_in_fibonacci_sequence(14, None).unwrap());
        assert_eq!(false, is_in_fibonacci_sequence(36, None).unwrap());
        assert_eq!(false, is_in_fibonacci_sequence(45, None).unwrap());
    }

    #[test]
    fn out_of_bounds_fibonacci_number() {
        assert!(is_in_fibonacci_sequence(53316291187, None).is_err())
    }
}

// 8 min 25 first implementation: normal recursion
// 3 min 48 second implementation: makes use of last result
// 2 ms implementation: make use of value caching
