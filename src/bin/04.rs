advent_of_code::solution!(4);

#[cfg(test)]
const LEN: usize = 10;
#[cfg(not(test))]
const LEN: usize = 139;

#[derive(Debug, Copy, Clone)]
struct Coordinate {
    index: usize,
    y: usize,
}

impl Coordinate {
    fn adjacent_indexes(&self) -> [Option<usize>; 3] {
        let mut adjacent = [None; 3];
        let mut insert_index = 0;

        {
            let y = self.y + 1;

            if y < LEN {
                adjacent[insert_index] = Some(self.index + 1);

                insert_index += 1;
            }
        }

        adjacent[insert_index] = Some(self.index);
        insert_index += 1;

        if 0 < self.y {
            adjacent[insert_index] = Some(self.index - 1);
        }

        adjacent
    }
}

fn parse_input(input: &str) -> ([(bool, u8); LEN * LEN], Vec<Coordinate>) {
    let mut grid = [(false, 0); LEN * LEN];
    let mut to_check = Vec::with_capacity(LEN * LEN);

    input.lines().rev().enumerate().for_each(|(y, line)| {
        for (x, elem) in line.chars().enumerate() {
            let index = y + (x * LEN);

            let hit = elem == '@';
            let mut cur_count = 0;

            if let Some(prev) = index
                .checked_sub(LEN)
                .map(|prev_x_index| &mut grid[prev_x_index])
            {
                if hit {
                    prev.1 += 1;
                }

                if prev.0 {
                    cur_count += 1;
                }
            }

            if hit {
                cur_count += 1;
                to_check.push(Coordinate { index, y });
            }

            grid[index] = (hit, cur_count);
        }
    });

    (grid, to_check)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (grid, to_check) = parse_input(input);
    let mut result = 0;

    for coordinate in to_check {
        let mut total = 0;

        let mut coords = coordinate.adjacent_indexes().into_iter();
        while let Some(Some(index)) = coords.next()
            && total < 5
        {
            total += grid[index].1;
        }

        // Less than 5 because the coordinate origin (not adjacent) is counted in the total
        if total < 5 {
            result += 1;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut grid, mut to_check) = parse_input(input);
    let mut to_remove = Vec::with_capacity(to_check.len());

    let mut result = 0;

    loop {
        for (to_check_index, coordinate) in to_check.iter().enumerate() {
            let mut total = 0;

            let adjacent_indexes = coordinate.adjacent_indexes();

            {
                let mut iter = adjacent_indexes.into_iter();
                while let Some(Some(index)) = iter.next()
                    && total < 5
                {
                    total += grid[index].1;
                }
            }

            // Less than 5 because the coordinate origin (not adjacent) is counted in the total
            if total < 5 {
                result += 1;

                if let Some(prev) = coordinate
                    .index
                    .checked_sub(LEN)
                    .map(|prev_x_index| &mut grid[prev_x_index])
                {
                    prev.1 -= 1;
                }

                grid[coordinate.index].1 -= 1;

                {
                    let next_index = coordinate.index + LEN;
                    if next_index < LEN * LEN {
                        grid[next_index].1 -= 1;
                    }
                }

                to_remove.push(to_check_index);
            }
        }

        if to_remove.is_empty() {
            break;
        } else {
            to_remove.sort_unstable();

            for index in to_remove.iter().copied().rev() {
                to_check.remove(index); // TODO - This is horrible for performance, fix
            }

            to_remove.clear();
        }
    }

    Some(result)
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
        assert_eq!(result, Some(43));
    }
}
