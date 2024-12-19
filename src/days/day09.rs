use std::collections::HashSet;

use super::Day;

pub struct Day09;

impl Day for Day09 {
    type Input = Vec<i64>;
    fn parse_input(input: &str) -> Self::Input {
        input.chars().map(|c| (c as i64) - 48).collect()
    }

    type OP1 = u64;
    fn part_1(input: Self::Input) -> Self::OP1 {
        let mut res = 0;

        let mut id = 0i64;
        let mut blocks = Vec::with_capacity(input.len());
        input.into_iter().enumerate().for_each(|(i, b)| {
            if i % 2 == 0 {
                for _ in 0..b {
                    blocks.push(id);
                }
                id += 1;
            } else {
                for _ in 0..b {
                    blocks.push(-1);
                }
            }
        });

        let (mut l, mut r) = (0usize, blocks.len() - 1);
        loop {
            if l >= r {
                break;
            }

            while blocks[r] == -1 {
                r -= 1;
                continue;
            }

            if blocks[l] == -1 {
                blocks[l] = blocks[r];
                blocks[r] = -1;
                r -= 1;
            }
            l += 1;
        }

        for (i, b) in blocks.into_iter().take_while(|i| *i != -1).enumerate() {
            res += (i as u64) * (b as u64);
        }

        res
    }

    type OP2 = u64;
    fn part_2(input: Self::Input) -> Self::OP2 {
        let mut res = 0;

        let mut id = 0i64;
        let mut blocks = Vec::with_capacity(input.len());
        input.into_iter().enumerate().for_each(|(i, b)| {
            if i % 2 == 0 {
                for _ in 0..b {
                    blocks.push(id);
                }
                id += 1;
            } else {
                for _ in 0..b {
                    blocks.push(-1);
                }
            }
        });

        let mut l = 0;
        while blocks[l] != -1 {
            l += 1;
        }

        let mut moved = HashSet::new();
        let mut r = blocks.len() - 1;
        while l < r {
            if blocks[r] == -1 {
                r -= 1;
                continue;
            }

            let id = blocks[r];
            let file_size = (0..=r).rev().take_while(|i| blocks[*i] == id).count();

            if moved.contains(&id) {
                r -= file_size;
                continue;
            }

            let mut i = l;
            loop {
                while blocks[i] != -1 {
                    i += 1;
                }

                if i > r - file_size {
                    r -= file_size;
                    break;
                }

                let free = blocks[i..].into_iter().take_while(|i| **i == -1).count();
                if free >= file_size {
                    for x in i..(i + file_size) {
                        blocks.swap(x, r);
                        r -= 1;
                    }
                    moved.insert(id);
                    break;
                }
                i += free;
            }
        }

        for (i, b) in blocks.into_iter().enumerate() {
            if b == -1 {
                continue;
            }
            res += (i as u64) * (b as u64);
        }

        res
    }
}

#[cfg(test)]
mod test09 {
    use crate::days::Day;

    const TEST_INPUT: &str = r#"2333133121414131402"#;

    #[test]
    fn part1() {
        let input = super::Day09::parse_input(TEST_INPUT);
        assert_eq!(super::Day09::part_1(input), 1928);
    }

    #[test]
    fn part2() {
        let input = super::Day09::parse_input(TEST_INPUT);
        assert_eq!(super::Day09::part_2(input), 2858);
    }
}
