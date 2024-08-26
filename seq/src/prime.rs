use num_traits::PrimInt;

pub struct Prime<N> {
    curr: N,
    next: N,
    trial1: N,
    trial2: N,
}

impl<N> Prime<N>
where
    N: PrimInt,
{
    #[must_use]
    pub fn new() -> Self {
        let one = N::one();
        let two = one + one;
        let three = one + two;
        let five = two + three;
        let seven = five + two;
        Self {
            curr: two,
            next: three,
            trial1: five,
            trial2: seven,
        }
    }
}

impl<N> Iterator for Prime<N>
where
    N: PrimInt,
{
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        let one = N::one();
        let two = one + one;
        let three = one + two;
        let six = three + three;
        let prime = self.curr;
        self.curr = self.next;
        loop {
            self.next = self.trial1;
            self.trial1 = self.trial2;
            self.trial2 = self.next + six;
            if is_prime(self.next) {
                break;
            }
        }
        Some(prime)
    }
}

impl<N> Default for Prime<N>
where
    N: PrimInt,
{
    fn default() -> Self {
        Self::new()
    }
}

#[allow(clippy::module_name_repetitions)]
pub fn is_prime<N: PrimInt>(candidate: N) -> bool {
    let (zero, one) = (N::zero(), N::one());
    let two = one + one;
    let three = one + two;
    if candidate <= one {
        return false;
    }
    if [two, three].contains(&candidate) {
        return true;
    }
    if candidate % two == zero {
        return false;
    }
    let try_limit = candidate / two;
    let mut n = three;
    loop {
        if candidate % n == zero {
            return false;
        }
        if n <= try_limit {
            n = n + two;
        } else {
            return true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_iter() {
        let mut primes: Prime<i32> = Prime::new();
        assert_eq!(primes.next(), Some(2));
        assert_eq!(primes.next(), Some(3));
        assert_eq!(primes.next(), Some(5));
        assert_eq!(primes.next(), Some(7));
        assert_eq!(primes.next(), Some(11));
        assert_eq!(primes.next(), Some(13));
        assert_eq!(primes.next(), Some(17));
        assert_eq!(primes.next(), Some(19));
        assert_eq!(primes.next(), Some(23));
        assert_eq!(primes.next(), Some(29));
        assert_eq!(primes.next(), Some(31));
    }
}
