advent_of_code::solution!(3);

#[cfg(not(test))]
const BANK_LEN: usize = 100;
#[cfg(test)]
const BANK_LEN: usize = 15;

fn extract_digits<const T: usize>(line: &str) -> [u8; T] {
    let mut digits = [0; T];

    for (i, char) in line.chars().enumerate() {
        digits[i] = char.to_digit(10).unwrap() as u8;
    }

    digits
}

fn combine_digits(a: u64, b: u64) -> u64 {
    (a * 10) + b
}

fn parse_input(input: &str) -> impl Iterator<Item = [u8; BANK_LEN]> {
    input.lines().map(extract_digits)
}

pub fn part_one(input: &str) -> Option<u64> {
    let banks = parse_input(input);
    let mut joltage = 0;

    for bank in banks {
        let mut working = (0, 0);

        for i in 0..(bank.len() - 1) {
            let a = bank[i];

            if a < working.0 {
                continue;
            }

            for b in bank[(i + 1)..].iter() {
                let b = *b;

                if working.0 < a || (working.0 == a && working.1 < b) {
                    working = (a, b);
                }
            }
        }

        joltage += combine_digits(working.0 as u64, working.1 as u64);
    }

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

    #[test]
    fn test_extract_digits() {
        assert_eq!(extract_digits("0123456789"), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
