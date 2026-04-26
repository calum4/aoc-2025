use std::str::{FromStr, Lines};

advent_of_code::solution!(5);

#[cfg(test)]
const RANGE_LEN: usize = 4;
#[cfg(not(test))]
const RANGE_LEN: usize = 182;

fn parse_input(input: &str) -> ([(u64, u64); RANGE_LEN], Lines<'_>) {
    let mut ranges = [(0, 0); RANGE_LEN];
    let mut lines = input.lines();

    for (index, line) in lines.by_ref().enumerate() {
        if line.is_empty() {
            break;
        }

        let mut split = line.split('-');

        let start = u64::from_str(split.next().unwrap()).unwrap();
        let end = u64::from_str(split.next().unwrap()).unwrap();

        ranges[index] = (start, end);
    }

    (ranges, lines)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, lines) = parse_input(input);
    let mut result = 0;

    for line in lines {
        let id = u64::from_str(line).unwrap();

        for range in ranges {
            if range.0 <= id && id <= range.1 {
                result += 1;
                break;
            }
        }
    }

    Some(result)
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
