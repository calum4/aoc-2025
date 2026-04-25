advent_of_code::solution!(4);

#[cfg(test)]
const LEN: usize = 10;
#[cfg(not(test))]
const LEN: usize = 139;

#[derive(Debug, Copy, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn adjacent_coords(&self) -> [Option<Coordinate>; 8] {
        let mut adjacent = [None; 8];
        let mut index = 0;

        // (x|y) <= 139
        let cur_y = self.y as i16;
        let cur_x = self.x as i16;

        for y_offset in [-1, 0, 1] {
            let y = cur_y + y_offset;

            if y < 0 || LEN as i16 <= y {
                continue;
            }

            for x_offset in [-1, 0, 1] {
                if y_offset == 0 && x_offset == 0 {
                    continue;
                }

                let x = cur_x + x_offset;

                if x < 0 || LEN as i16 <= x {
                    continue;
                }

                adjacent[index] = Some(Coordinate {
                    x: x as usize,
                    y: y as usize,
                });

                index += 1;
            }
        }

        adjacent
    }
}

fn parse_input(input: &str) -> ([[bool; LEN]; LEN], Vec<Coordinate>) {
    let mut grid = [[false; LEN]; LEN];
    let mut to_check = Vec::with_capacity(LEN * LEN);

    input.lines().rev().enumerate().for_each(|(y, line)| {
        for (x, elem) in line.chars().enumerate() {
            if elem == '@' {
                grid[y][x] = true;
                to_check.push(Coordinate { x, y });
            }
        }
    });

    (grid, to_check)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (grid, to_check) = parse_input(input);
    let mut result = 0;

    for coordinate in to_check {
        let mut total = 0;

        let mut coords = coordinate.adjacent_coords().into_iter();
        while let Some(Some(coord)) = coords.next()
            && total < 4
        {
            if grid[coord.y][coord.x] {
                total += 1;
            }
        }

        if total < 4 {
            result += 1;
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
