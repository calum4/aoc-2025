advent_of_code::solution!(2);

// static to avoid duplication across memory
static POW_10: [u64; 20] = calc_pow_10();

const fn calc_pow_10<const T: usize>() -> [u64; T] {
    let mut pow_10s = [1; T];

    let mut i = 1;

    while i < T {
        pow_10s[i] = u64::saturating_mul(pow_10s[i - 1], 10);
        i += 1;
    }

    pow_10s
}

fn count_base_10_digits(n: u64) -> usize {
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

fn base_10_split_in_half(n: u64, digit_count: usize) -> (u64, u64) {
    let pow = POW_10[digit_count / 2];
    (n / pow, n % pow)
}

fn base_10_combine(a: u64, b: u64) -> u64 {
    let b_digits = count_base_10_digits(b);
    (a * (POW_10[b_digits])) + b
}

fn extract_ranges_with_even_number_of_digits(
    result: &mut Vec<(usize, u64, u64)>,
    low: u64,
    high: u64,
) {
    let mut start_digits = count_base_10_digits(low);

    // select starting point with even number of digits, e.g. 4000 is 4 digits
    let mut start = if start_digits.is_multiple_of(2) {
        low
    } else {
        let new_low = POW_10[start_digits];
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
        let end = (POW_10[start_digits] - 1).min(high);
        result.push((start_digits, start, end));

        start_digits += 2;
        start = POW_10[start_digits - 1]
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

        for (digits, boundary_start, boundary_end) in test_ranges.iter().cloned() {
            let start = {
                let (a, b) = base_10_split_in_half(boundary_start, digits);
                a.min(b)
            };

            let end = {
                let (a, b) = base_10_split_in_half(boundary_end, digits);
                a.max(b)
            };

            for half in start..=end {
                let num = base_10_combine(half, half);
                if boundary_start <= num && num <= boundary_end {
                    invalid_sum += num;
                }
            }
        }
    }

    Some(invalid_sum)
}

fn base_10_split_digits(digits: &mut Vec<u64>, mut n: u64, digit_count: usize) {
    digits.resize(digit_count, 0);

    for i in 1..=digit_count {
        digits[digit_count - i] = n % 10;
        n /= 10;
    }
}

fn contains_pattern(split_digits: &[u64]) -> bool {
    let digits = split_digits.len();

    for k in 1..=(digits / 2) {
        if !digits.is_multiple_of(k) {
            continue;
        }

        let mut ok = true;

        for i in 0..digits {
            if split_digits[i] != split_digits[i % k] {
                ok = false;
                break;
            }
        }

        if ok {
            return true;
        }
    }

    false
}

// TODO - Optimise
pub fn part_two(input: &str) -> Option<u64> {
    let mut invalid_sum = 0;
    let mut split_digits = Vec::new();

    for (low, high) in parse_input(input) {
        for n in low..=high {
            let digits = count_base_10_digits(n);
            base_10_split_digits(&mut split_digits, n, digits);

            if contains_pattern(&split_digits) {
                invalid_sum += n;
            }
        }
    }

    Some(invalid_sum)
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
        assert_eq!(result, Some(4174379265));
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
        assert_eq!(base_10_split_in_half(10, 2), (1, 0));
        assert_eq!(base_10_split_in_half(1432, 4), (14, 32));
        assert_eq!(base_10_split_in_half(1010, 4), (10, 10));
        assert_eq!(base_10_split_in_half(59124592, 8), (5912, 4592));
    }

    #[test]
    fn test_combine_base_10() {
        assert_eq!(base_10_combine(0, 1), 1);
        assert_eq!(base_10_combine(1, 0), 10);
        assert_eq!(base_10_combine(1, 1), 11);
        assert_eq!(base_10_combine(389, 24), 38924);
    }

    #[test]
    fn test_extract_ranges_with_even_number_of_digits() {
        let mut vec = Vec::new();

        extract_ranges_with_even_number_of_digits(&mut vec, 0, 10);
        assert_eq!(vec, vec![(2, 10, 10)]);

        vec.clear();
        extract_ranges_with_even_number_of_digits(&mut vec, 0, 100);
        assert_eq!(vec, vec![(2, 10, 99)]);

        vec.clear();
        extract_ranges_with_even_number_of_digits(&mut vec, 13, 100);
        assert_eq!(vec, vec![(2, 13, 99)]);

        vec.clear();
        extract_ranges_with_even_number_of_digits(&mut vec, 4531, 777777);
        assert_eq!(vec, vec![(4, 4531, 9999), (6, 100000, 777777)]);
    }

    #[test]
    fn test_base_10_split_digits() {
        let mut vec = Vec::new();
        base_10_split_digits(&mut vec, 5299153, 7);
        assert_eq!(vec, vec![5, 2, 9, 9, 1, 5, 3]);

        vec.clear();
        base_10_split_digits(&mut vec, 13, 2);
        assert_eq!(vec, vec![1, 3]);

        base_10_split_digits(&mut vec, 1, 1);
        assert_eq!(vec, vec![1]);

        base_10_split_digits(&mut vec, 3729211618202, 13);
        assert_eq!(vec, vec![3, 7, 2, 9, 2, 1, 1, 6, 1, 8, 2, 0, 2]);
    }

    #[test]
    fn test_contains_pattern() {
        assert!(contains_pattern(&vec![1, 1]));
        assert!(!contains_pattern(&vec![1, 2]));

        assert!(contains_pattern(&vec![1, 2, 1, 2]));
        assert!(!contains_pattern(&vec![1, 2, 2, 2]));
        assert!(contains_pattern(&vec![2, 2, 2, 2]));

        assert!(contains_pattern(&vec![1, 2, 3, 1, 2, 3]));
        assert!(contains_pattern(&vec![1, 2, 1, 2, 1, 2]));
        assert!(!contains_pattern(&vec![1, 2, 1, 2, 1, 1]));

        assert!(contains_pattern(&vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7]));
        assert!(contains_pattern(&vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2]));
        assert!(contains_pattern(&vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5]));
    }
}
