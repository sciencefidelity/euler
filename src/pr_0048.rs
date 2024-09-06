pub struct Euler;

impl Euler {
    pub fn self_powers(_target: usize) -> usize {
        let result = vec![0; 10];

        Self::vec_to_int(result)
    }

    fn pow_long(n: Vec<u8>) -> Vec<u8> {
        let (mut result, mut carry) = (vec![0; 10], 0);
        println!("i\tdigit\tcarry\tprod\tn");
        for (i, digit) in result.iter_mut().enumerate().rev() {
            let prod = n[i] * n[i] + carry;
            *digit = prod % 10;
            carry = prod / 10;
            println!("{}\t{}\t{}\t{}\t{}", i, digit, carry, prod, n[i]);
        }
        result
    }

    fn vec_to_int(vec: Vec<u8>) -> usize {
        let mut result = 0;
        for i in vec {
            result *= 10;
            result += usize::from(i);
        }
        result
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

// #[test]
// fn test_self_powers_ten() {
//     assert_eq!(Euler::self_powers(10), 405_071_317);
// }

// #[test]
// fn test_self_powers_thousand() {
//     assert_eq!(Euler::self_powers(1000), 1);
// }

// #[test]
// fn test_long_pow() {
//     assert_eq!(
//         Euler::pow_long(vec![0, 0, 0, 0, 9, 8, 7, 6, 5, 4]),
//         vec![5, 4, 6, 0, 4, 2, 3, 7, 1, 6]
//     );
// }

// #[test]
// fn test_long_pow() {
//     assert_eq!(
//         Euler::pow_long(vec![0, 0, 0, 0, 0, 0, 0, 0, 5, 4]),
//         vec![0, 0, 0, 0, 0, 0, 2, 9, 1, 6]
//     );
// }

// #[test]
// fn test_vec_to_int() {
//     assert_eq!(
//         Euler::vec_to_int(vec![0, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
//         234567890
//     );
// }
// }

/*

   54
 * 54
-----
  2916
-----
  21

 16
200
200
2500

81
810
8100
810
*/
