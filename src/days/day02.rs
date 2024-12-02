use super::Day;

pub struct Day02;

impl Day02 {
    fn is_level_safe(level: &Vec<i64>) -> bool {
        if level.len() < 2 {
            return true;
        }

        let order = (level[0] - level[1]) > 0;

        for i in 0..(level.len() - 1) {
            let diff = level[i] - level[i + 1];
            if diff.abs() > 3 || diff.abs() < 1 {
                return false;
            }

            let curr_order = diff > 0;
            if curr_order != order {
                return false;
            }
        }
        true
    }

    fn dampen_error_safe(level: &Vec<i64>) -> bool {
        for i in 0..level.len() {
            let mut level = level.clone();
            level.remove(i);
            if Self::is_level_safe(&level) {
                return true;
            }
        }
        false
    }
}

impl Day for Day02 {
    type Input = Vec<Vec<i64>>;
    fn parse_input(input: &str) -> Self::Input {
        let mut res = Vec::<Vec<i64>>::new();
        input.lines().for_each(|l| {
            let levels = l
                .split(' ')
                .map(|i| i.parse().expect("Failed to parse level to u64"))
                .collect();
            res.push(levels);
        });
        res
    }

    type OP1 = u64;
    fn part_1(input: &Self::Input) -> Self::OP1 {
        input.into_iter().fold(0, |acc, level| {
            if Self::is_level_safe(level) {
                acc + 1
            } else {
                acc
            }
        })
    }

    type OP2 = u64;
    fn part_2(input: &Self::Input) -> Self::OP2 {
        input.into_iter().fold(0, |acc, level| {
            if Self::is_level_safe(level) || Self::dampen_error_safe(level) {
                acc + 1
            } else {
                acc
            }
        })
    }
}
