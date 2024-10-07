pub const fn longest_collatz_sequence(n: usize) -> usize {
    let (mut output, mut longest_chain, mut i) = (1, 0, n / 2);
    while i < n {
        let len = count_chain(i);
        if len > longest_chain {
            longest_chain = len;
            output = i;
        }
        i += 1;
    }
    output
}

const fn count_chain(mut n: usize) -> usize {
    let mut count = 1;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
            count += 1;
        } else {
            n = (3 * n + 1) / 2;
            count += 2;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_collatz_sequence() {
        assert_eq!(837_799, longest_collatz_sequence(1_000_000));
    }
}
