use std::cmp::{max, min};
use std::iter::zip;

fn parse_input(input: String) -> (Vec<u64>, Vec<u64>) {
    let mut left_list = Vec::<u64>::new();
    let mut right_list = Vec::<u64>::new();
    input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(left, right)| {
            (
                left.trim().parse::<u64>().unwrap(),
                right.trim().parse::<u64>().unwrap(),
            )
        })
        .for_each(|(left, right)| {
            left_list.push(left);
            right_list.push(right);
        });
    (left_list, right_list)
}

pub fn part_one(input: String) -> u64 {
    let mut answer = 0;
    let (mut left_list, mut right_list) = parse_input(input);
    left_list.sort();
    right_list.sort();
    for (left, right) in zip(left_list, right_list) {
        answer += max(left, right) - min(left, right);
    }
    answer
}

pub fn part_two(input: String) -> u64 {
    let mut answer = 0;
    let (left_list, right_list) = parse_input(input);
    left_list.iter().for_each(|left| {
        let count = right_list.iter().filter(|r| *r == left).count() as u64;
        answer += left * count;
    });
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = String::from(
            "3   4
4   3
2   5
1   3
3   9
3   3
",
        );
        assert_eq!(part_one(input), 11)
    }

    #[test]
    fn test_part_two() {
        let input = String::from(
            "3   4
4   3
2   5
1   3
3   9
3   3
",
        );
        assert_eq!(part_two(input), 31)
    }
}
