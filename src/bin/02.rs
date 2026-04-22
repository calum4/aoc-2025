use std::ops::RangeInclusive;

advent_of_code::solution!(2);

fn count_base_10_digits(n: u64) -> u32 {
    let mut comp = 10;
    let mut count = 1;

    loop {
        if n < comp {
            break;
        }

        let (value, overflow) = comp.overflowing_mul(10);
        if overflow {
            break;
        }

        comp = value;
        count += 1;
    }

    count
}

fn split_base_10_in_half(n: u64, digit_count: u32) -> (u64, u64) {
    let pow = 10u64.pow(digit_count / 2);
    (n / pow, n % pow)
}

fn extract_ranges_with_even_number_of_digits(
    result: &mut Vec<(u32, RangeInclusive<u64>)>,
    low: u64,
    high: u64,
) {
    let mut start_digits = count_base_10_digits(low);

    // select starting point with even number of digits, e.g. 4000 is 4 digits
    let mut start = if start_digits.is_multiple_of(2) {
        low
    } else {
        let new_low = 10u64.pow(start_digits);
        start_digits += 1;

        if high < new_low {
            return;
        }

        new_low
    };

    loop {
        if high < start {
            break;
        }

        // calculate last valid number with `start_digits` number of digits
        let end = (10u64.pow(start_digits) - 1).min(high);
        result.push((start_digits, start..=end));

        start_digits += 2;
        start = 10u64.pow(start_digits - 1)
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = (u64, u64)> {
    input.split(',').map(|s| {
        let mut split = s.split('-');

        #[inline]
        fn extract(split: &mut std::str::Split<char>) -> u64 {
            split.next().unwrap().parse::<u64>().unwrap()
        }

        (extract(&mut split), extract(&mut split))
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut invalid_sum = 0;

    let mut test_ranges = Vec::new();

    for (low, high) in parse_input(input) {
        test_ranges.clear();
        extract_ranges_with_even_number_of_digits(&mut test_ranges, low, high);

        for (digits, range) in test_ranges.iter().cloned() {
            for i in range {
                let (a, b) = split_base_10_in_half(i, digits);
                if a == b {
                    invalid_sum += i;
                }
            }
        }
    }

    Some(invalid_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_count_base_10_digits() {
        assert_eq!(count_base_10_digits(0), 1);
        assert_eq!(count_base_10_digits(1), 1);
        assert_eq!(count_base_10_digits(5), 1);
        assert_eq!(count_base_10_digits(9), 1);
        assert_eq!(count_base_10_digits(10), 2);
        assert_eq!(count_base_10_digits(13), 2);
        assert_eq!(count_base_10_digits(20), 2);
        assert_eq!(count_base_10_digits(97), 2);
        assert_eq!(count_base_10_digits(99), 2);
        assert_eq!(count_base_10_digits(100), 3);
        assert_eq!(count_base_10_digits(103), 3);
        assert_eq!(count_base_10_digits(999), 3);
        assert_eq!(count_base_10_digits(7821), 4);
        assert_eq!(count_base_10_digits(88888), 5);
        assert_eq!(count_base_10_digits(721589), 6);
        assert_eq!(count_base_10_digits(3578169442254158893), 19);
        assert_eq!(count_base_10_digits(u64::MAX - 1), 19);
        assert_eq!(count_base_10_digits(u64::MAX), 19);
    }

    #[test]
    fn test_split_base_10_in_half() {
        assert_eq!(split_base_10_in_half(10, 2), (1, 0));
        assert_eq!(split_base_10_in_half(1432, 4), (14, 32));
        assert_eq!(split_base_10_in_half(1010, 4), (10, 10));
        assert_eq!(split_base_10_in_half(59124592, 8), (5912, 4592));
    }

    #[test]
    fn test_extract_ranges_with_even_number_of_digits() {
        let mut vec = Vec::new();

        extract_ranges_with_even_number_of_digits(&mut vec, 0, 10);
        assert_eq!(vec, vec![(2, 10..=10)]);

        vec.clear();
        extract_ranges_with_even_number_of_digits(&mut vec, 0, 100);
        assert_eq!(vec, vec![(2, 10..=99)]);

        vec.clear();
        extract_ranges_with_even_number_of_digits(&mut vec, 13, 100);
        assert_eq!(vec, vec![(2, 13..=99)]);

        vec.clear();
        extract_ranges_with_even_number_of_digits(&mut vec, 4531, 777777);
        assert_eq!(vec, vec![(4, 4531..=9999), (6, 100000..=777777)]);
    }
}
