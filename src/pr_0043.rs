const PRIMES: [u64; 7] = [2, 3, 5, 7, 11, 13, 17];

pub fn sub_string_divisibility() -> u64 {
    let mut result = 0;
    backtrack(&mut result, &mut Vec::with_capacity(10));
    result
}

fn backtrack(result: &mut u64, pan: &mut Vec<u64>) {
    if pan.len() == 10 && is_sub_string_divisible(pan) {
        *result += join(pan);
    }
    for i in 0..=9 {
        if pan.contains(&i) {
            continue;
        }
        pan.push(i);
        backtrack(result, pan);
        pan.pop();
    }
}

fn is_sub_string_divisible(v: &[u64]) -> bool {
    let mut is_divisible = true;
    for (w, prime) in v.windows(3).skip(1).zip(PRIMES) {
        if join(w) % prime != 0 {
            is_divisible = false;
            break;
        }
    }
    is_divisible
}

fn join(v: &[u64]) -> u64 {
    let mut res = 0;
    for (i, n) in v.iter().rev().enumerate() {
        res += n * 10_u64.pow(u32::try_from(i).unwrap());
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_string_divisibility() {
        assert_eq!(sub_string_divisibility(), 16_695_334_890);
    }

    #[test]
    fn test_join() {
        assert_eq!(join(&[4, 0, 6]), 406);
        assert_eq!(join(&[0, 6, 3]), 63);
        assert_eq!(join(&[0, 6, 3]), 63);
    }

    #[test]
    fn test_is_substring_divisible() {
        assert!(is_sub_string_divisible(&[1, 4, 0, 6, 3, 5, 7, 2, 8, 9]),);
    }
}
