advent_of_code::solution!(3);

fn combine_digits(a: u64, b: u64) -> u64 {
    (a * 10) + b
}

pub fn part_one(input: &str) -> Option<u64> {
    let joltage = input
        .lines()
        .map(|line| line.as_bytes())
        .map(|bytes| {
            let mut working = [0, 0];

            for i in 0..(bytes.len() - 1) {
                let a = bytes[i];

                if a < working[0] {
                    continue;
                }

                for b in bytes[(i + 1)..].iter() {
                    let b = *b;

                    if working[0] < a || (working[0] == a && working[1] < b) {
                        working = [a, b];
                    }
                }
            }

            working.iter_mut().for_each(|i| *i -= b'0');

            combine_digits(working[0] as u64, working[1] as u64)
        })
        .sum();

    Some(joltage)
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
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
