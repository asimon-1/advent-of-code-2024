use std::collections::VecDeque;

const PADDING: i32 = 5;

const DIRECTION: [(i32, i32); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

fn parse_input(input: String) -> VecDeque<VecDeque<char>> {
    let mut ret: VecDeque<VecDeque<char>> = input
        .lines()
        .map(|line| {
            let mut l = line.chars().collect::<VecDeque<char>>();
            for _ in 0..PADDING {
                l.push_front('.');
                l.push_back('.');
            }
            l
        })
        .collect();
    let width = ret[0].len();
    for _ in 0..PADDING {
        ret.push_back(vec!['.'; width].into());
        ret.push_front(vec!['.'; width].into());
    }
    ret
}

pub fn part_one(input: String) -> u32 {
    let table = parse_input(input);
    let width = table[0].len() as i32;
    let height = table.len() as i32;
    let mut answer = 0;
    for x in PADDING..(width - PADDING) {
        for y in PADDING..(height - PADDING) {
            if table[y as usize][x as usize] == 'X' {
                for (xp, yp) in DIRECTION {
                    if table[(y + yp) as usize][(x + xp) as usize] == 'M'
                        && table[(y + 2 * yp) as usize][(x + 2 * xp) as usize] == 'A'
                        && table[(y + 3 * yp) as usize][(x + 3 * xp) as usize] == 'S'
                    {
                        answer += 1;
                    }
                }
            }
        }
    }
    answer
}

pub fn part_two(input: String) -> u32 {
    let table = parse_input(input);
    let width = table[0].len() as i32;
    let height = table.len() as i32;
    let mut answer = 0;
    for x in PADDING..(width - PADDING) {
        for y in PADDING..(height - PADDING) {
            if table[y as usize][x as usize] == 'A' {
                'dir: for (xp, yp) in DIRECTION {
                    if xp == 0 || yp == 0 {
                        // Don't consider cardinal directions
                        continue;
                    }
                    if table[(y + yp) as usize][(x + xp) as usize] == 'M'
                        && table[(y - yp) as usize][(x - xp) as usize] == 'S'
                        && ((table[(y + yp) as usize][(x - xp) as usize] == 'M'
                            && table[(y - yp) as usize][(x + xp) as usize] == 'S')
                            || (table[(y - yp) as usize][(x + xp) as usize] == 'M'
                                && table[(y + yp) as usize][(x - xp) as usize] == 'S'))
                    {
                        answer += 1;
                        break 'dir;
                    }
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
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );
        assert_eq!(part_one(input), 18)
    }

    #[test]
    fn test_part_two() {
        let input = String::from(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );
        assert_eq!(part_two(input), 9)
    }
}
