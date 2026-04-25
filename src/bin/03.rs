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
    let joltage = input
        .lines()
        .map(|line| line.as_bytes())
        .map(|bytes| {
            let mut result: u64 = 0;
            let mut start = 0usize;

            for t in 0..12 {
                let last = bytes.len() - (12 - t);
                let mut best = b'0';
                let mut best_idx = start;

                for (i, byte) in bytes.iter().enumerate().take(last + 1).skip(start) {
                    if *byte > best {
                        best = *byte;
                        best_idx = i;

                        if best == b'9' {
                            break;
                        }
                    }
                }

                result = result * 10 + (best - b'0') as u64;
                start = best_idx + 1;
            }

            result
        })
        .sum();

    Some(joltage)
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
        assert_eq!(result, Some(3121910778619));
    }
}
