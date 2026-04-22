advent_of_code::solution!(2);

fn count_base_10_digits(n: u64) -> u64 {
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

pub fn part_one(input: &str) -> Option<u64> {
    let mut invalid_sum = 0;

    for range_str in input.split(',') {
        let (low, high) = {
            let mut split = range_str.split('-');

            #[inline]
            fn extract(split: &mut std::str::Split<char>) -> u64 {
                split.next().unwrap().parse::<u64>().unwrap()
            }

            (extract(&mut split), extract(&mut split))
        };

        for i in low..=high {
            let digits = count_base_10_digits(i);

            if !digits.is_multiple_of(2) {
                continue;
            }

            let (a, b) = split_base_10_in_half(i, digits as u32);
            if a == b {
                invalid_sum += i;
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
}
