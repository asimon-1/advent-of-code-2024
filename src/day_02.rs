use std::cmp::{max, min};

fn parse_input(input: String) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|l| l.split(" ").map(|x| x.parse::<u64>().unwrap()).collect())
        .collect()
}

fn is_safe(report: &[u64]) -> bool {
    (report.iter().is_sorted_by(|a, b| a < b) || report.iter().is_sorted_by(|a, b| a > b))
        && report
            .windows(2)
            .all(|window| (max(window[0], window[1]) - min(window[0], window[1]) <= 3))
}

fn generate_subreports(report: &[u64]) -> Vec<Vec<u64>> {
    let mut subreports = Vec::new();
    subreports.push(report.to_owned());
    for ind in 0..report.len() {
        let mut subreport = report.to_owned();
        subreport.remove(ind);
        subreports.push(subreport);
    }
    subreports
}

pub fn part_one(input: String) -> u64 {
    let table = parse_input(input);
    table.iter().filter(|report| is_safe(report)).count() as u64
}

pub fn part_two(input: String) -> u64 {
    let table = parse_input(input);
    table
        .iter()
        .filter(|report| {
            let subreports = generate_subreports(report);
            subreports.iter().any(|x| is_safe(x))
        })
        .count() as u64
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
