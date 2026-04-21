use std::str::FromStr;

advent_of_code::solution!(1);

/// Dial position must be between 0-99 inclusive, and wrap
/// I.E, 99 + 2 = 1
#[derive(Debug, Copy, Clone)]
struct Dial(usize);

impl Default for Dial {
    fn default() -> Self {
        Dial(50)
    }
}

impl PartialEq<usize> for Dial {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

impl Dial {
    const MAX: usize = 99;

    /// # Returns
    /// Number of times the dial passed zero
    fn sub(&mut self, rhs: usize) -> u64 {
        let mut passed_zero = (rhs / (Self::MAX + 1)) as u64;
        let started_at_zero = self.0 == 0;

        let (mut result, overflow) = self.0.overflowing_sub(rhs % (Self::MAX + 1));

        if overflow {
            result = Self::MAX - (usize::MAX - result);
            passed_zero += 1;
        }

        if result == 0 {
            passed_zero += 1;
        }

        self.0 = result;

        if started_at_zero {
            passed_zero = passed_zero.saturating_sub(1);
        }

        passed_zero
    }

    /// # Returns
    /// Number of times the dial passed zero
    fn add(&mut self, rhs: usize) -> u64 {
        let mut passed_zero = (rhs / (Self::MAX + 1)) as u64;
        let mut result = self.0 + (rhs % (Self::MAX + 1));

        if Self::MAX < result {
            result = (result - Self::MAX).saturating_sub(1);
            passed_zero += 1;
        }

        self.0 = result;

        passed_zero
    }

    /// # Returns
    /// Number of times the dial passed zero
    fn apply(&mut self, direction: Direction, distance: usize) -> u64 {
        match direction {
            Direction::Left => self.sub(distance),
            Direction::Right => self.add(distance),
        }
    }
}

enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unimplemented!(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut zero_count = 0;
    let mut dial = Dial::default();

    for line in input.lines() {
        let direction = Direction::from(line.as_bytes()[0] as char);
        let distance = usize::from_str(&line[1..]).unwrap();

        dial.apply(direction, distance);
        if dial == 0 {
            zero_count += 1;
        }
    }

    Some(zero_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut zero_count = 0;
    let mut dial = Dial::default();

    for line in input.lines() {
        let direction = Direction::from(line.as_bytes()[0] as char);
        let distance = usize::from_str(&line[1..]).unwrap();

        zero_count += dial.apply(direction, distance);
    }

    Some(zero_count)
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_dial_sub() {
        {
            let mut dial = Dial(50);
            assert_eq!(dial.sub(68), 1);
            assert_eq!(dial, 82);
        }

        {
            let mut dial = Dial(82);
            assert_eq!(dial.sub(30), 0);
            assert_eq!(dial, 52);
        }

        {
            let mut dial = Dial(0);
            assert_eq!(dial.sub(5), 0);
            assert_eq!(dial, 95);
        }

        {
            let mut dial = Dial(55);
            assert_eq!(dial.sub(55), 1);
            assert_eq!(dial, 0);
        }

        {
            let mut dial = Dial(0);
            assert_eq!(dial.sub(1), 0);
            assert_eq!(dial, 99);
        }

        {
            let mut dial = Dial(99);
            assert_eq!(dial.sub(99), 1);
            assert_eq!(dial, 0);
        }

        {
            let mut dial = Dial(14);
            assert_eq!(dial.sub(82), 1);
            assert_eq!(dial, 32);
        }

        {
            let mut dial = Dial(14);
            assert_eq!(dial.sub(105), 1);
            assert_eq!(dial, 9);

            assert_eq!(dial.sub(337), 4);
            assert_eq!(dial, 72);
        }
    }

    #[test]
    fn test_dial_add() {
        {
            let mut dial = Dial(52);
            assert_eq!(dial.add(48), 1);
            assert_eq!(dial, 0);
        }

        {
            let mut dial = Dial(95);
            assert_eq!(dial.add(60), 1);
            assert_eq!(dial, 55);
        }

        {
            let mut dial = Dial(0);
            assert_eq!(dial.add(14), 0);
            assert_eq!(dial, 14);
        }

        {
            let mut dial = Dial(0);
            assert_eq!(dial.add(140), 1);
            assert_eq!(dial, 40);

            assert_eq!(dial.add(351), 3);
            assert_eq!(dial, 91);

            assert_eq!(dial.add(110), 2);
            assert_eq!(dial, 1);
        }
    }
}
