use std::cmp::{max, min};

fn parse_input(input: String) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.split(" ").map(|x| x.parse::<u32>().unwrap()).collect())
        .collect()
}

fn is_safe(report: &Vec<u32>) -> bool {
    (report.iter().is_sorted_by(|a, b| a < b) || report.iter().is_sorted_by(|a, b| a > b))
        && report
            .windows(2)
            .all(|window| (max(window[0], window[1]) - min(window[0], window[1]) <= 3))
}

fn generate_subreports(report: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut subreports = Vec::new();
    subreports.push(report.clone());
    for ind in 0..report.len() {
        let mut subreport = report.clone();
        subreport.remove(ind);
        subreports.push(subreport);
    }
    subreports
}

pub fn part_one(input: String) -> u32 {
    let table = parse_input(input);
    table.iter().filter(|report| is_safe(*report)).count() as u32
}

pub fn part_two(input: String) -> u32 {
    let table = parse_input(input);
    table
        .iter()
        .filter(|report| {
            let subreports = generate_subreports(report);
            subreports.iter().any(is_safe)
        })
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = String::from(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        assert_eq!(part_one(input), 2)
    }

    #[test]
    fn test_part_two() {
        let input = String::from(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        assert_eq!(part_two(input), 4)
    }
}
