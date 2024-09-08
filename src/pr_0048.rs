pub struct Euler;

impl Euler {
    pub fn self_powers(target: usize) -> usize {
        let mut powers = vec![];
        for i in 1..=target {
            powers.push(Self::pow(i));
        }
        let result = Self::add(powers);
        Self::vec_to_int(&result[result.len() - 10..])
    }

    fn multiply(a: Vec<u32>, b: Vec<u32>) -> Vec<u32> {
        let (mut working, mut max_len) = (Vec::new(), 0);
        for (idx, i) in a.iter().rev().enumerate() {
            let (mut tmp, mut carry) = (Vec::new(), 0);
            for _ in 0..idx {
                tmp.push(0);
            }
            for j in b.iter().rev() {
                let prod = i * j + carry;
                tmp.push(prod % 10);
                carry = prod / 10;
            }
            while carry > 0 {
                tmp.push(carry % 10);
                carry /= 10;
            }
            max_len = max_len.max(tmp.len());
            working.push(tmp);
        }
        let (mut result, mut carry) = (Vec::new(), 0);
        for i in 0..=max_len {
            let mut sum = 0;
            for j in &working {
                sum += *j.get(i).unwrap_or(&0);
            }
            sum += carry;
            result.push(sum % 10);
            carry = sum / 10;
        }
        result.reverse();
        result
    }

    fn pow(n: usize) -> Vec<u32> {
        let a = Self::int_to_vec(n);
        let mut b = a.clone();
        for _ in 1..n {
            b = Self::multiply(a.clone(), b);
        }
        b
    }

    fn add(mut arr: Vec<Vec<u32>>) -> Vec<u32> {
        arr.iter_mut().for_each(|x| x.reverse());
        let n = arr[arr.len() - 1].len();
        let (mut result, mut carry) = (Vec::new(), 0);
        for i in 0..n {
            let mut sum = 0;
            for j in &arr {
                sum += *j.get(i).unwrap_or(&0);
            }
            sum += carry;
            result.push(sum % 10);
            carry = sum / 10;
        }
        result.reverse();
        result
    }

    fn int_to_vec(mut n: usize) -> Vec<u32> {
        let mut result = Vec::new();
        while n > 0 {
            result.push(u32::try_from(n % 10).unwrap());
            n /= 10;
        }
        result.reverse();
        result
    }

    fn vec_to_int(vec: &[u32]) -> usize {
        let mut result = 0;
        for i in vec.into_iter() {
            result *= 10;
            result += usize::try_from(*i).unwrap();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_powers_ten() {
        assert_eq!(Euler::self_powers(10), 405_071_317);
    }

    #[test]
    fn test_self_powers_13() {
        assert_eq!(Euler::self_powers(14), 3_749_340_453);
    }

    #[test]
    fn test_self_powers_thousand() {
        assert_eq!(Euler::self_powers(1000), 9_110_846_700);
    }

    #[test]
    fn test_multiply_a() {
        assert_eq!(
            Euler::multiply(vec![9, 8, 7, 6, 5, 4], vec![9, 8, 7, 6, 5, 4]),
            vec![0, 9, 7, 5, 4, 6, 0, 4, 2, 3, 7, 1, 6]
        );
    }

    #[test]
    fn test_multiply_b() {
        assert_eq!(Euler::multiply(vec![5, 4], vec![5, 4]), vec![0, 2, 9, 1, 6]);
    }

    #[test]
    fn test_pow_b() {
        assert_eq!(Euler::pow(9), vec![0, 3, 8, 7, 4, 2, 0, 4, 8, 9]);
    }

    #[test]
    fn test_vec_to_int() {
        assert_eq!(
            Euler::vec_to_int(&[0, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
            234_567_890
        );
    }

    #[test]
    fn test_int_to_vec() {
        assert_eq!(Euler::int_to_vec(1234), vec![1, 2, 3, 4]);
    }
}
