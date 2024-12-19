use super::Day;
use std::collections::HashSet;

pub struct Day10;

impl Day10 {
    fn traverse(
        grid: &Vec<Vec<i32>>,
        prev: i32,
        (i, j): (usize, usize),
        set: &mut HashSet<(usize, usize)>,
    ) -> i32 {
        let next = prev + 1;

        let mut res = 0;
        if i < (grid.len() - 1) && grid[i + 1][j] == next {
            if next == 9 && !set.contains(&(i + 1, j)) {
                set.insert((i + 1, j));
                res += 1;
            }
            res += Self::traverse(grid, next, (i + 1, j), set);
        }
        if j < (grid[i].len() - 1) && grid[i][j + 1] == next {
            if next == 9 && !set.contains(&(i, j + 1)) {
                res += 1;
                set.insert((i, j + 1));
            }
            res += Self::traverse(grid, next, (i, j + 1), set);
        }
        if i > 0 && grid[i - 1][j] == next {
            if next == 9 && !set.contains(&(i - 1, j)) {
                res += 1;
                set.insert((i - 1, j));
            }
            res += Self::traverse(grid, next, (i - 1, j), set);
        }
        if j > 0 && grid[i][j - 1] == next {
            if next == 9 && !set.contains(&(i, j - 1)) {
                res += 1;
                set.insert((i, j - 1));
            }
            res += Self::traverse(grid, next, (i, j - 1), set);
        }

        res
    }

    fn traverse_distinct(grid: &Vec<Vec<i32>>, prev: i32, (i, j): (usize, usize)) -> i32 {
        let next = prev + 1;

        let mut res = 0;
        if i < (grid.len() - 1) && grid[i + 1][j] == next {
            res += if next == 9 {
                1
            } else {
                Self::traverse_distinct(grid, next, (i + 1, j))
            };
        }
        if j < (grid[i].len() - 1) && grid[i][j + 1] == next {
            res += if next == 9 {
                1
            } else {
                Self::traverse_distinct(grid, next, (i, j + 1))
            };
        }
        if i > 0 && grid[i - 1][j] == next {
            res += if next == 9 {
                1
            } else {
                Self::traverse_distinct(grid, next, (i - 1, j))
            };
        }
        if j > 0 && grid[i][j - 1] == next {
            res += if next == 9 {
                1
            } else {
                Self::traverse_distinct(grid, next, (i, j - 1))
            };
        }

        res
    }
}

impl Day for Day10 {
    type Input = Vec<Vec<i32>>;
    fn parse_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(|s| s.chars().map(|c| (c as i32) - 48).collect())
            .collect()
    }

    type OP1 = i32;
    fn part_1(input: Self::Input) -> Self::OP1 {
        let mut res = 0;

        for i in 0..input.len() {
            for j in 0..input[i].len() {
                if input[i][j] == 0 {
                    let mut set = HashSet::new();
                    res += Self::traverse(&input, 0, (i, j), &mut set);
                }
            }
        }

        res
    }

    type OP2 = i32;
    fn part_2(input: Self::Input) -> Self::OP2 {
        let mut res = 0;

        for i in 0..input.len() {
            for j in 0..input[i].len() {
                if input[i][j] == 0 {
                    res += Self::traverse_distinct(&input, 0, (i, j));
                }
            }
        }

        res
    }
}
