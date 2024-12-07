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

fn transform_input(input: Vec<Vec<char>>) -> Vec<Vec<Position>> {
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

fn step(input: &mut Vec<Vec<Position>>, current_pos: &mut Position) -> Option<bool> {
    // Returns Some(true) if we have been in this position before (loop detected)
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
        step(input, current_pos)
    } else {
        input[current_pos.y as usize][current_pos.x as usize] = *current_pos;
        *current_pos = next_pos;
        Some(false)
    }
}

fn walk(simulation: &mut Vec<Vec<Position>>, starting_pos: Position) -> bool {
    let mut current_pos = starting_pos;
    while let Some(is_loop) = step(simulation, &mut current_pos) {
        if is_loop {
            return true;
        }
    }
    // Mark our position one last time, since we exited step() without having done so
    simulation[current_pos.y as usize][current_pos.x as usize] = current_pos;
    false
}

pub fn part_one(input: String) -> u64 {
    let input = parse_input(input);
    let starting_pos = find_initial_position(&input);
    let mut input = transform_input(input);
    walk(&mut input, starting_pos);
    input
        .iter()
        .map(|row| {
            row.iter()
                .filter(|c| [UP, DOWN, LEFT, RIGHT].contains(&c.dir))
                .count()
        })
        .sum::<usize>() as u64
}

pub fn part_two(input: String) -> u64 {
    let input = parse_input(input);
    let starting_pos = find_initial_position(&input);
    let input = transform_input(input);
    // Walk the path once first so we can identify all the locations where we need to place an obstruction
    // Placing an obstruction not on the path will never re-route the path, so don't need to evaluate those cases
    let mut unobstructed_path = input.clone();
    walk(&mut unobstructed_path, starting_pos);
    let mut answer = 0;
    for row_idx in 0..input.len() {
        for col_idx in 0..input[row_idx].len() {
            if [UP, DOWN, LEFT, RIGHT].contains(&unobstructed_path[row_idx][col_idx].dir)
                && input[row_idx][col_idx] != starting_pos
            {
                let mut simulation = input.clone();
                simulation[row_idx][col_idx].dir = OBSTRUCTION;
                if walk(&mut simulation, starting_pos) {
                    answer += 1;
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
