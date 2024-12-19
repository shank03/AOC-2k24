use std::collections::HashMap;

use super::Day;

pub trait Stone {
    fn mutate(&self) -> Result<(u64, u64), u64>;
    fn digits(&self) -> u64;
}
impl Stone for u64 {
    fn mutate(&self) -> Result<(u64, u64), u64> {
        let res = self.digits();
        if res % 2 != 0 {
            return Err(*self);
        }

        let a = self / 10u64.pow(res as u32 / 2);
        let b = self % 10u64.pow(res as u32 / 2);

        Ok((a, b))
    }

    fn digits(&self) -> u64 {
        (self.ilog10() + 1) as u64
    }
}

pub struct Day11;

impl Day11 {
    fn blinks(nums: &Vec<u64>, iters: usize) -> u64 {
        let mut map = HashMap::<u64, u64>::new();
        nums.into_iter().for_each(|i| {
            map.insert(*i, 1);
        });

        for _ in 0..iters {
            for (s, v) in map.drain().collect::<Vec<_>>() {
                let mut insert = |k: u64| {
                    map.entry(k).and_modify(|n| *n += v).or_insert(v);
                };

                if s == 0 {
                    insert(1);
                    continue;
                }

                match s.mutate() {
                    Ok((a, b)) => {
                        insert(a);
                        insert(b);
                    }
                    Err(x) => insert(x * 2024),
                };
            }
        }

        map.values().sum()
    }
}

impl Day for Day11 {
    type Input = Vec<u64>;
    fn parse_input(input: &str) -> Self::Input {
        input
            .split(' ')
            .map(|s| s.trim().parse().unwrap())
            .collect()
    }

    type OP1 = u64;
    fn part_1(input: Self::Input) -> Self::OP1 {
        Self::blinks(&input, 25)
    }

    type OP2 = u64;
    fn part_2(input: Self::Input) -> Self::OP2 {
        Self::blinks(&input, 75)
    }
}
