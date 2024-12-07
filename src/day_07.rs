enum Operation {
    Add,
    Multiply,
    Concat,
}

impl Operation {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
            Operation::Concat => format!("{a}{b}").parse::<u64>().unwrap(),
        }
    }
}

fn parse_input(input: String) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| line.split_once(":").unwrap())
        .map(|(tvalue, vals)| {
            let test_value = tvalue.parse::<u64>().unwrap();
            let nums: Vec<u64> = vals.trim().split(" ").map(|v| v.parse().unwrap()).collect();
            (test_value, nums)
        })
        .collect()
}

fn can_work(test_val: u64, nums: &[u64], operations: &[Operation]) -> bool {
    let mut stack = vec![(0, nums[0])];
    while let Some((depth, total)) = stack.pop() {
        if total > test_val {
            // Value is too big, don't process any more
            continue;
        }
        if depth == nums.len() - 1 {
            if total == test_val {
                // We processed all the numbers and the calculation total is correct
                return true;
            }
        } else {
            // There's still calculation to do
            // Add each of the operations to the stack for the next number
            stack.extend(
                operations
                    .iter()
                    .map(|operation| (depth + 1, operation.apply(total, nums[depth + 1]))),
            );
        }
    }
    false
}

pub fn part_one(input: String) -> u64 {
    let input = parse_input(input);
    input
        .iter()
        .filter(|(test_val, nums)| {
            can_work(*test_val, nums, &[Operation::Add, Operation::Multiply])
        })
        .map(|(test_val, _)| test_val)
        .sum()
}

pub fn part_two(input: String) -> u64 {
    let input = parse_input(input);
    input
        .iter()
        .filter(|(test_val, nums)| {
            can_work(
                *test_val,
                nums,
                &[Operation::Add, Operation::Multiply, Operation::Concat],
            )
        })
        .map(|(test_val, _)| test_val)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = String::from(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        );
        assert_eq!(part_one(input), 3749)
    }

    #[test]
    fn test_part_two() {
        let input = String::from(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        );
        assert_eq!(part_two(input), 11387)
    }
}
