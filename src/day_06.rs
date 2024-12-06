const UP: (i32, i32) = (-1, 0);
const DOWN: (i32, i32) = (1, 0);
const LEFT: (i32, i32) = (0, -1);
const RIGHT: (i32, i32) = (0, 1);
const EMPTY: (i32, i32) = (0, 0);
const OBSTRUCTION: (i32, i32) = (i32::MAX, i32::MAX);

#[derive(PartialEq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
    dir: (i32, i32),
}

impl Position {
    fn rotate(&mut self) {
        self.dir = match self.dir {
            UP => RIGHT,
            RIGHT => DOWN,
            DOWN => LEFT,
            LEFT => UP,
            _ => unreachable!(),
        };
    }
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn transform_input_part_two(input: Vec<Vec<char>>) -> Vec<Vec<Position>> {
    input
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| match *c {
                    '#' => Position {
                        x: 0,
                        y: 0,
                        dir: OBSTRUCTION,
                    },
                    _ => Position {
                        x: 0,
                        y: 0,
                        dir: EMPTY,
                    },
                })
                .collect()
        })
        .collect()
}

fn find_initial_position(input: &[Vec<char>]) -> Position {
    let mut pos = Position {
        x: 0,
        y: 0,
        dir: UP,
    };
    input.iter().enumerate().for_each(|(y, row)| {
        if let Some(x) = row.iter().position(|c| *c == '^') {
            pos.x = x as i32;
            pos.y = y as i32;
        }
    });
    pos
}

fn step_part_one(input: &mut Vec<Vec<char>>, current_pos: &mut Position) -> Option<()> {
    // Returns Some(()) if we successfully took a step
    // Returns None if we exited the map
    let next_pos = Position {
        x: current_pos.x + current_pos.dir.1,
        y: current_pos.y + current_pos.dir.0,
        dir: current_pos.dir,
    };
    let next_char = input.get(next_pos.y as usize)?.get(next_pos.x as usize)?;
    match next_char {
        '#' => {
            current_pos.rotate();
            step_part_one(input, current_pos)
        }
        _ => {
            input[current_pos.y as usize][current_pos.x as usize] = 'X';
            input[next_pos.y as usize][next_pos.x as usize] = '^';
            *current_pos = next_pos;
            Some(())
        }
    }
}

fn step_part_two(input: &mut Vec<Vec<Position>>, current_pos: &mut Position) -> Option<bool> {
    // Returns Some(true) if we have been in this position before
    // Returns Some(false) if we haven't been in this position before and we succesfully took a step
    // Returns None if we exited the map

    if input[current_pos.y as usize][current_pos.x as usize] == *current_pos {
        return Some(true);
    }
    let next_pos = Position {
        x: current_pos.x + current_pos.dir.1,
        y: current_pos.y + current_pos.dir.0,
        dir: current_pos.dir,
    };
    let next_location = input.get(next_pos.y as usize)?.get(next_pos.x as usize)?;
    if next_location.dir == OBSTRUCTION {
        current_pos.rotate();
        step_part_two(input, current_pos)
    } else {
        input[current_pos.y as usize][current_pos.x as usize] = *current_pos;
        *current_pos = next_pos;
        Some(false)
    }
}

pub fn part_one(input: String) -> u32 {
    let mut input = parse_input(input);
    let mut current_pos = find_initial_position(&input);
    while step_part_one(&mut input, &mut current_pos).is_some() {}
    input
        .iter()
        .map(|row| row.iter().filter(|c| **c == 'X').count())
        .sum::<usize>() as u32
        + 1 // Add one for the last step before exiting
}

pub fn part_two(input: String) -> u32 {
    let input = parse_input(input);
    let starting_pos = find_initial_position(&input);
    let mut answer = 0;
    for row_idx in 0..input.len() {
        for col_idx in 0..input[row_idx].len() {
            let mut simulation = transform_input_part_two(input.clone());
            let mut current_pos = starting_pos;
            if simulation[row_idx][col_idx] != current_pos {
                simulation[row_idx][col_idx].dir = OBSTRUCTION;
            }
            while let Some(is_loop) = step_part_two(&mut simulation, &mut current_pos) {
                if is_loop {
                    answer += 1;
                    break;
                }
            }
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = String::from(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        );
        assert_eq!(part_one(input), 41)
    }

    #[test]
    fn test_part_two() {
        let input = String::from(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        );
        assert_eq!(part_two(input), 6)
    }
}
