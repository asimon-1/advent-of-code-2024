use regex::Regex;

#[derive(Debug)]
struct Order {
    before: String,
    after: String,
}

impl Order {
    fn from_delimited(line: &str) -> Order {
        let (before, after) = line.split_once("|").expect("No delimiter |");
        Order {
            before: before.to_string(),
            after: after.to_string(),
        }
    }

    fn to_regex(&self) -> Regex {
        let pattern = format!("{},.*?{}", self.after, self.before);
        Regex::new(&pattern).expect("Could not compile regex")
    }
}

fn parse_input(input: String) -> (Vec<Order>, Vec<String>) {
    let (rules_str, update_str) = input.split_once("\n\n").expect("No delimiter \\n\\n");
    let rules = rules_str.lines().map(Order::from_delimited).collect();
    let updates = update_str.lines().map(|x| x.to_string()).collect();
    (rules, updates)
}

pub fn part_one(input: String) -> u32 {
    let (rules, updates) = parse_input(input);
    let patterns: Vec<Regex> = rules.iter().map(|rule| rule.to_regex()).collect();
    updates
        .iter()
        .filter(|update| patterns.iter().all(|p| !p.is_match(update)))
        .map(|u| {
            let pages = u.split(",").collect::<Vec<&str>>();
            pages[(pages.len() - 1) / 2].parse::<u32>().unwrap()
        })
        .sum()
}

pub fn part_two(input: String) -> u32 {
    let (rules, updates) = parse_input(input);
    let patterns: Vec<Regex> = rules.iter().map(|rule| rule.to_regex()).collect();
    updates
        .iter()
        .filter(|update| patterns.iter().any(|p| p.is_match(update)))
        .map(|update| update.split(",").collect::<Vec<&str>>())
        .map(|update| {
            let mut u = update.clone();
            'outer: loop {
                for order in &rules {
                    if let Some(before_idx) = u.iter().position(|x| *x == order.before) {
                        if let Some(after_idx) = u.iter().position(|x| *x == order.after) {
                            if before_idx > after_idx {
                                u.swap(before_idx, after_idx);
                                // We might now violate some earlier rule so start again at the beginning
                                // Praying that there's no infinite loop here
                                continue 'outer;
                            }
                        }
                    }
                }
                // If we made it through all the rules without needing to swap anything, then we are sorted and can move on.
                break;
            }
            u
        })
        .map(|pages| pages[(pages.len() - 1) / 2].parse::<u32>().unwrap())
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
