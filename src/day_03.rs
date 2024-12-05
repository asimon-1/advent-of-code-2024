use regex::{Match, Regex};

pub fn part_one(input: String) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(&input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap())
        .sum()
}

pub fn part_two(input: String) -> u32 {
    let re = Regex::new(r"do\(\)([\s\S]*?)don't\(\)").unwrap();
    let mut instructions = String::from("do()");
    instructions.push_str(&input);
    instructions.push_str("don't()");
    re.captures_iter(&instructions)
        .map(|c| c.extract())
        .map(|(_, [inner])| part_one(inner.to_string()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input =
            String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(part_one(input), 161)
    }

    #[test]
    fn test_part_two() {
        let input = String::from(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        assert_eq!(part_two(input), 48)
    }
}
