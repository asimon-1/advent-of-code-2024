pub fn part_one(input: String) -> u32 {
    let answer = 42;
    answer
}

pub fn part_two(input: String) -> u32 {
    let answer = 43;
    answer
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
