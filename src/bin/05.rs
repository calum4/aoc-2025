use std::str::FromStr;

advent_of_code::solution!(5);

#[cfg(test)]
const RANGE_LEN: usize = 4;
#[cfg(not(test))]
const RANGE_LEN: usize = 182;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn contains(&self, value: u64) -> bool {
        self.start <= value && value <= self.end
    }

    /// # Returns
    /// `true` if merge was successful, `false` if not
    fn merge(&mut self, other: &Self) -> bool {
        if other.start <= self.end.saturating_add(1) && other.end >= self.start.saturating_sub(1) {
            self.start = self.start.min(other.start);
            self.end = self.end.max(other.end);

            true
        } else {
            false
        }
    }
}

fn parse_input(input: &str) -> (Vec<Range>, impl Iterator<Item = u64> + '_) {
    let mut ranges = [Range::default(); RANGE_LEN];
    let mut lines = input.lines();

    for (index, line) in lines.by_ref().enumerate() {
        if line.is_empty() {
            break;
        }

        let mut split = line.split('-');
        let start = u64::from_str(split.next().unwrap()).unwrap();
        let end = u64::from_str(split.next().unwrap()).unwrap();

        ranges[index] = Range { start, end };
    }

    ranges.sort_unstable_by(|a, b| a.start.cmp(&b.start));

    let mut merged = Vec::with_capacity(ranges.len());
    merged.push(ranges[0]);

    for next in ranges.into_iter().skip(1) {
        if let Some(prev) = merged.last_mut()
            && prev.merge(&next)
        {
            continue;
        }

        merged.push(next);
    }

    let ids = lines.map(|line| u64::from_str(line).unwrap());

    (merged, ids)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ids) = parse_input(input);
    let mut result = 0;

    for id in ids {
        for range in &ranges {
            if range.contains(id) {
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

    #[test]
    fn test_range_merge() {
        {
            let a = Range { start: 1, end: 3 };
            let b = Range { start: 1, end: 5 };

            let mut a2 = a;
            assert!(a2.merge(&b));

            let res = Range { start: 1, end: 5 };
            assert_eq!(a2, res);
        }

        {
            let a = Range { start: 1, end: 3 };
            let b = Range { start: 2, end: 7 };

            let mut a2 = a;
            assert!(a2.merge(&b));

            let res = Range { start: 1, end: 7 };
            assert_eq!(a2, res);
        }

        {
            let a = Range { start: 8, end: 9 };
            let b = Range { start: 0, end: 7 };

            let mut a2 = a;
            assert!(a2.merge(&b));

            let res = Range { start: 0, end: 9 };
            assert_eq!(a2, res);
        }

        {
            let a = Range { start: 5, end: 9 };
            let b = Range { start: 0, end: 7 };

            let mut a2 = a;
            assert!(a2.merge(&b));

            let res = Range { start: 0, end: 9 };
            assert_eq!(a2, res);
        }

        {
            let a = Range { start: 1, end: 2 };
            let b = Range { start: 3, end: 4 };

            let mut a2 = a;
            assert!(a2.merge(&b));

            let res = Range { start: 1, end: 4 };
            assert_eq!(a2, res);
        }

        {
            let a = Range { start: 1, end: 2 };
            let b = Range { start: 4, end: 5 };

            let mut a2 = a;
            assert!(!a2.merge(&b));

            let res = Range { start: 1, end: 2 };
            assert_eq!(a2, res);
        }
    }
}
