pub struct Euler;

const MONTHS: [usize; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const YEAR: usize = 365;
const WEEK: usize = 7;

impl Euler {
    pub fn counting_sundays(start: usize, end: usize) -> usize {
        let mut count = 0;
        let mut iter = (1..=7).cycle();
        for year in start..=end {
            let mut months = MONTHS;
            if Self::is_leap_year(year) {
                months[2] += 1;
            }
            for month in months {
                for day_of_month in 1..=month {
                    let day_of_week = iter.next().unwrap();
                    if day_of_week == 7 && day_of_month == 1 && year != 1900 {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    const fn is_leap_year(y: usize) -> bool {
        y % 400 == 0 || y % 100 != 0 && y % 4 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(171, Euler::counting_sundays(1900, 2000));
    }

    #[test]
    fn leap_year_2024() {
        assert!(Euler::is_leap_year(2024));
    }

    #[test]
    fn leap_year_2021() {
        assert!(!Euler::is_leap_year(2021));
    }

    #[test]
    fn leap_year_1900() {
        assert!(!Euler::is_leap_year(1900));
    }

    #[test]
    fn leap_year_2000() {
        assert!(Euler::is_leap_year(2000));
    }
}
