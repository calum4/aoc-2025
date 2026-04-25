advent_of_code::solution!(3);

fn combine_digits(a: u64, b: u64) -> u64 {
    (a * 10) + b
}

pub fn part_one(input: &str) -> Option<u64> {
    let joltage = input
        .lines()
        .map(|line| line.as_bytes())
        .map(|bytes| {
            let mut best = (b'0', bytes[bytes.len() - 1]);

            for byte in bytes[..bytes.len() - 1].iter().copied().rev() {
                if byte > best.0 {
                    if best.0 > best.1 {
                        best.1 = best.0;
                    }

                    best.0 = byte;
                } else if byte == best.0 && byte > best.1 {
                    best.1 = best.0;
                }
            }

            best = (best.0 - b'0', best.1 - b'0');

            combine_digits(best.0 as u64, best.1 as u64)
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
