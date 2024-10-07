pub fn integer_right_triangles(target: i32) -> i32 {
    let (mut result, mut max_count) = (0, 0);
    for i in 1..=target {
        let count = find_count_int_right_triangles(i);
        if count > max_count {
            max_count = count;
            result = i;
        }
    }
    result
}

fn find_count_int_right_triangles(p: i32) -> i32 {
    let (mut count, max_ab) = (0, 2 * (p / 3));
    for b in 1..=max_ab {
        for a in 1..=b {
            let sum_ab = a + b;
            let c_limit = p - sum_ab;
            for c in b..=c_limit {
                if a + b + c == p && is_pythagorean_triplet((a, b, c)) {
                    count += 1;
                }
            }
        }
    }
    count
}

const fn is_pythagorean_triplet(triplet: (i32, i32, i32)) -> bool {
    triplet.0.pow(2) + triplet.1.pow(2) == triplet.2.pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_right_triangles() {
        assert_eq!(integer_right_triangles(1000), 840);
    }

    #[test]
    fn test_count_int_rignt_triangles() {
        assert_eq!(find_count_int_right_triangles(120), 3);
    }
}
