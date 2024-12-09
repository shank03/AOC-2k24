use super::Day;

pub struct Day07;

impl Day07 {
    fn dp(target: u64, operands: &Vec<u64>, prev: u64, idx: usize) -> bool {
        if idx == operands.len() {
            return prev == target;
        }

        Self::dp(target, operands, prev + operands[idx], idx + 1)
            || Self::dp(target, operands, prev * operands[idx], idx + 1)
    }

    fn dp_or(target: u64, operands: &Vec<u64>, prev: u64, idx: usize) -> bool {
        if idx == operands.len() {
            return prev == target;
        }

        Self::dp_or(target, operands, prev + operands[idx], idx + 1)
            || Self::dp_or(target, operands, prev * operands[idx], idx + 1)
            || Self::dp_or(
                target,
                operands,
                format!("{prev}{}", operands[idx]).parse().unwrap(),
                idx + 1,
            )
    }
}

impl Day for Day07 {
    type Input = Vec<(u64, Vec<u64>)>;
    fn parse_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(|s| {
                let mut tokens = s.split(':');
                let target = tokens.next().unwrap().trim().parse().unwrap();
                let operands = tokens
                    .last()
                    .unwrap()
                    .trim()
                    .split(' ')
                    .map(|i| i.trim().parse().unwrap())
                    .collect();

                (target, operands)
            })
            .collect()
    }

    type OP1 = u64;
    fn part_1(input: &Self::Input) -> Self::OP1 {
        let mut res = 0;

        for (target, operands) in input.into_iter() {
            if Self::dp(*target, operands, operands[0], 1) {
                res += target;
            }
        }

        res
    }

    type OP2 = u64;
    fn part_2(input: &Self::Input) -> Self::OP2 {
        let mut res = 0;

        for (target, operands) in input.into_iter() {
            if Self::dp_or(*target, operands, operands[0], 1) {
                res += target;
            }
        }

        res
    }
}

#[cfg(test)]
mod test07 {
    use crate::days::Day;

    const TEST_INPUT: &str = r#"190: 10 19
    3267: 81 40 27
    83: 17 5
    156: 15 6
    7290: 6 8 6 15
    161011: 16 10 13
    192: 17 8 14
    21037: 9 7 18 13
    292: 11 6 16 20"#;

    #[test]
    fn part1() {
        let input = super::Day07::parse_input(TEST_INPUT);
        assert_eq!(super::Day07::part_1(&input), 3749);
    }
}
