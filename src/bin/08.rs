// THIS CODE IS HORRIFIC AND I'M ASHAMED OF IT. YOU'VE BEEN WARNED.

use itertools::Itertools;

advent_of_code::solution!(8);

#[cfg(test)]
const PAIRS: usize = 10;
#[cfg(not(test))]
const PAIRS: usize = 1000;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Coordinate {
    x: u64,
    y: u64,
    z: u64,
}

impl From<[u64; 3]> for Coordinate {
    fn from([x, y, z]: [u64; 3]) -> Self {
        Self { x, y, z }
    }
}

impl Coordinate {
    fn distance(&self, other: &Self) -> u64 {
        fn diff_pow(a: u64, b: u64) -> u64 {
            a.abs_diff(b).pow(2)
        }

        (diff_pow(self.x, other.x) + diff_pow(self.y, other.y) + diff_pow(self.z, other.z)).isqrt()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut junction_boxes = Vec::new();
    let mut xyz = [0; 3];

    for line in input.lines() {
        for (index, val) in line.split(',').take(3).enumerate() {
            xyz[index] = val.parse::<u64>().unwrap();
        }

        junction_boxes.push(Coordinate::from(xyz));
    }

    let mut junction_distances = junction_boxes
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (*a, *b, a.distance(b)))
        .collect::<Vec<(Coordinate, Coordinate, u64)>>();

    let (sorted_junction_distances, _, _) =
        junction_distances.select_nth_unstable_by_key(PAIRS, |value| value.2);
    let mut circuits: Vec<Vec<Coordinate>> = Vec::new();

    fn find_circuit_index(circuits: &[Vec<Coordinate>], coordinate: &Coordinate) -> Option<usize> {
        for (index, circuit) in circuits.iter().enumerate() {
            if circuit.contains(coordinate) {
                return Some(index);
            }
        }

        None
    }

    for (a, b, _) in sorted_junction_distances {
        let a_index = find_circuit_index(&circuits, a);
        let b_index = find_circuit_index(&circuits, b);

        match (a_index, b_index) {
            (Some(a_index), Some(b_index)) => {
                circuits.push(Vec::new());
                let b_circuit = circuits.swap_remove(b_index);
                circuits[a_index].extend(b_circuit);
            }
            (Some(a_index), None) => {
                circuits[a_index].push(*b);
            }
            (None, Some(b_index)) => {
                circuits[b_index].push(*a);
            }
            (None, None) => {
                circuits.push(vec![*a, *b]);
            }
        }
    }

    let mut circuit_sizes: Vec<_> = circuits
        .iter()
        .map(|circuit| circuit.len() as u64)
        .collect();
    circuit_sizes.sort_unstable();

    Some(circuit_sizes.iter().rev().take(3).product())
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
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
