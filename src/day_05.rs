use itertools::Itertools;
use std::cmp::Ordering;

fn parse_input(input: String) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let (rules_str, update_str) = input.split_once("\n\n").expect("No delimiter \\n\\n");
    let rules = rules_str
        .lines()
        .map(|line| {
            line.split_once("|")
                .expect("Could not find rule delimiter!")
        })
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .collect();
    let updates = update_str
        .lines()
        .map(|update| update.split(","))
        .map(|update| update.map(|page| page.parse::<u32>().unwrap()).collect())
        .collect();
    (rules, updates)
}

fn compare(a: u32, b: u32, rules: &[(u32, u32)]) -> Ordering {
    if rules.contains(&(a, b)) {
        Ordering::Less
    } else if rules.contains(&(b, a)) {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn is_vec_equal(a: &[u32], b: &[u32]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(a, b)| a == b)
}

pub fn part_one(input: String) -> u32 {
    let (rules, updates) = parse_input(input);
    updates
        .iter()
        .filter(|update| {
            let mut sorted_update = (*update).clone();
            sorted_update.sort_by(|a, b| compare(*a, *b, &rules));
            is_vec_equal(update, &sorted_update)
        })
        .map(|update| update[(update.len() - 1) / 2])
        .sum()
}

pub fn part_two(input: String) -> u32 {
    let (rules, mut updates) = parse_input(input);
    updates
        .iter_mut()
        .filter(|update| {
            let mut sorted_update = (*update).clone();
            sorted_update.sort_by(|a, b| compare(*a, *b, &rules));
            !is_vec_equal(update, &sorted_update)
        })
        .update(|update| {
            update.sort_by(|a, b| compare(*a, *b, &rules));
        })
        .map(|u| u[(u.len() - 1) / 2])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = String::from(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );
        assert_eq!(part_one(input), 143)
    }

    #[test]
    fn test_part_two() {
        let input = String::from(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );
        assert_eq!(part_two(input), 123)
    }
}
