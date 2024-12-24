use std::collections::{HashMap, HashSet, VecDeque};

use super::Day;

pub struct Day19;

impl Day19 {
    fn is_valid(towel: &str, tokens: &[String]) -> bool {
        if towel.is_empty() {
            return true;
        }

        let mut queue: VecDeque<&str> = VecDeque::new();
        let mut visited: HashSet<&str> = HashSet::new();

        queue.push_back(towel);
        visited.insert(towel);

        while let Some(current) = queue.pop_front() {
            if current.is_empty() {
                return true;
            }

            for token in tokens {
                if current.starts_with(token) {
                    let next = &current[token.len()..];
                    if !visited.contains(next) {
                        queue.push_back(next);
                        visited.insert(next);
                    }
                }
            }
        }

        false
    }

    fn get_ways(
        towel: &str,
        pos: usize,
        tokens: &[String],
        max_tok: usize,
        mp: &mut HashMap<usize, u64>,
    ) -> u64 {
        let mut res = 0;

        if let Some(count) = mp.get(&pos) {
            return *count;
        }

        for tok in (0..max_tok).filter_map(|i| {
            if i + pos < towel.len() {
                Some(towel[pos..=pos + i].to_owned())
            } else {
                None
            }
        }) {
            if tokens.contains(&tok) {
                let pos = pos + tok.len();
                res += if pos == towel.len() {
                    1
                } else {
                    Self::get_ways(towel, pos, tokens, max_tok, mp)
                };
            }
        }

        mp.insert(pos, res);
        res
    }
}

impl Day for Day19 {
    type Input = (Vec<String>, Vec<String>);
    fn parse_input(input: &str) -> Self::Input {
        let mut l = input.lines();
        let mut tokens = l
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        tokens.sort();
        l.next();

        (tokens, l.map(|s| s.to_owned()).collect())
    }

    type OP1 = u64;
    fn part_1((tokens, combos): Self::Input) -> Self::OP1 {
        let mut res = 0;
        for towel in combos {
            res += if Self::is_valid(&towel, &tokens) {
                1
            } else {
                0
            };
        }

        res
    }

    type OP2 = u64;
    fn part_2((tokens, combos): Self::Input) -> Self::OP2 {
        let mut res = 0;

        let max_tok = combos.iter().map(|p| p.len()).max().unwrap();
        for towel in combos {
            let mut mp = HashMap::new();
            res += Self::get_ways(&towel, 0, &tokens, max_tok, &mut mp);
        }

        res
    }
}
