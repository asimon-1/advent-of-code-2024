pub fn part_one(input: String) -> u64 {
    42
}

pub fn part_two(input: String) -> u64 {
    43
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = String::from("");
        assert_eq!(part_one(input), 42)
    }

    #[test]
    fn test_part_two() {
        let input = String::from("");
        assert_eq!(part_two(input), 43)
    }
}
