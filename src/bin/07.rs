advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let (line_len, start_index) = {
        let line = lines.next().unwrap();
        (line.len(), line.find('S').unwrap())
    };

    let mut beam_indexes = Vec::with_capacity(line_len);
    beam_indexes.push(start_index);

    let mut working = Vec::with_capacity(line_len);
    let mut result = 0;

    for bytes in lines.map(|line| line.as_bytes()).skip(1).step_by(2) {
        for index in beam_indexes.iter().copied() {
            if bytes[index] == b'^' {
                let mut did_split = false;

                if let (left, overflow) = index.overflowing_sub(1)
                    && !overflow
                    && working.last().copied() != Some(left)
                {
                    working.push(left);
                    did_split = true;
                }

                if let right = index + 1
                    && right < line_len
                    && working.last().copied() != Some(right)
                {
                    working.push(right);
                    did_split = true;
                }

                if did_split {
                    result += 1;
                }
            } else if working.last().copied() != Some(index) {
                working.push(index);
            }
        }

        beam_indexes.clone_from(&working);
        working.clear();
    }

    Some(result)
}

fn apply_beam(working: &mut Vec<(usize, u64)>, index: usize, count: u64) {
    if let Some(last) = working.last_mut()
        && last.0 == index
    {
        last.1 += count;
    } else {
        working.push((index, count));
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let (line_len, start_index) = {
        let line = lines.next().unwrap();
        (line.len(), line.find('S').unwrap())
    };

    let mut beam_indexes = Vec::with_capacity(line_len);
    beam_indexes.push((start_index, 1u64));

    let mut working: Vec<(usize, u64)> = Vec::with_capacity(line_len);

    for bytes in lines.map(|line| line.as_bytes()).skip(1).step_by(2) {
        for (index, count) in beam_indexes.iter().copied() {
            if bytes[index] == b'^' {
                if let (left, overflow) = index.overflowing_sub(1)
                    && !overflow
                {
                    apply_beam(&mut working, left, count);
                }

                if let right = index + 1
                    && right < line_len
                {
                    apply_beam(&mut working, right, count);
                }
            } else {
                apply_beam(&mut working, index, count);
            }
        }

        beam_indexes.clone_from(&working);
        working.clear();
    }

    let result = beam_indexes.iter().map(|(_, count)| count).sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
