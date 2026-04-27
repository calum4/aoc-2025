use std::str::FromStr;
use wide::u64x8;

advent_of_code::solution!(6);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operation {
    Add,
    Mul,
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
    impl Iterator<Item = impl Iterator<Item = (usize, u64)> + '_>,
) {
    let mut lines = input.lines().map(str::split_ascii_whitespace);

    let operations = lines
        .next_back()
        .unwrap()
        .map(Operation::from_str)
        .map(Result::unwrap)
        .collect::<Vec<Operation>>();

    let number_rows = lines.map(|split| split.map(u64::from_str).map(Result::unwrap).enumerate());

    (operations, number_rows)
}

const CHUNK_SIZE: usize = 8;

pub fn part_one(input: &str) -> Option<u64> {
    let (operations, number_rows) = parse_input(input);
    let total_chunks = operations.len() / CHUNK_SIZE
        + if operations.len() % CHUNK_SIZE == 0 {
            0
        } else {
            1
        };

    let mut results = Vec::with_capacity(total_chunks);

    for operations_chunk in operations.chunks(CHUNK_SIZE) {
        let mut chunk = [0u64; CHUNK_SIZE];

        for (index, operation) in operations_chunk.iter().enumerate() {
            chunk[index] = match operation {
                Operation::Add => 0,
                Operation::Mul => 1,
            };
        }

        results.push(u64x8::new(chunk));
    }

    for row in number_rows {
        let mut chunk_index = 0;
        let mut insert_index = 0;

        let mut add_chunk = [0u64; CHUNK_SIZE];
        let mut mul_chunk = [1u64; CHUNK_SIZE];

        for (column_index, number) in row {
            let operation: Operation = operations[column_index];

            match operation {
                Operation::Add => {
                    add_chunk[insert_index] = number;
                }
                Operation::Mul => {
                    mul_chunk[insert_index] = number;
                }
            }

            if insert_index == mul_chunk.len() - 1 {
                results[chunk_index] += u64x8::new(add_chunk);
                add_chunk = [0; CHUNK_SIZE];

                results[chunk_index] = results[chunk_index] * u64x8::new(mul_chunk);
                mul_chunk = [1; CHUNK_SIZE];

                chunk_index += 1;
                insert_index = 0;
            } else {
                insert_index += 1;
            }
        }

        if insert_index != 0 {
            results[chunk_index] += u64x8::new(add_chunk);
            results[chunk_index] = results[chunk_index] * u64x8::new(mul_chunk);
        }
    }

    let sum: u64 = results
        .iter()
        .map(|chunk| chunk.to_array().iter().sum::<u64>())
        .sum();
    Some(sum)
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
