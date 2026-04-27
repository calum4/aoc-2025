use std::str::FromStr;

advent_of_code::solution!(6);

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add,
    Mul,
}

impl Operation {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Mul => a * b,
        }
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Operation::Add,
            "*" => Operation::Mul,
            _ => return Err(()),
        })
    }
}

fn parse_input(
    input: &str,
) -> (
    Vec<Operation>,
    impl Iterator<Item = impl Iterator<Item = (usize, u64)>>,
) {
    let mut lines = input.lines().rev().map(str::split_ascii_whitespace);

    let operations = lines
        .next()
        .unwrap()
        .map(Operation::from_str)
        .map(Result::unwrap)
        .collect::<Vec<Operation>>();

    let number_rows = lines.map(|split| split.map(u64::from_str).map(Result::unwrap).enumerate());

    (operations, number_rows)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (operations, number_rows) = parse_input(input);

    let mut results = Vec::with_capacity(operations.len());
    for operation in &operations {
        results.push(match operation {
            Operation::Add => 0,
            Operation::Mul => 1,
        });
    }

    for row in number_rows {
        for (index, number) in row {
            let operation = operations[index];

            results[index] = operation.apply(results[index], number);
        }
    }

    Some(results.iter().sum())
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
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
