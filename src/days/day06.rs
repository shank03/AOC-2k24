use std::collections::HashSet;

use super::Day;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Day06;

impl Day06 {
    fn find_loop(grid: &Vec<Vec<char>>, (mut gi, mut gj): (usize, usize)) -> bool {
        let mut dir = Direction::Up;
        let mut coords = HashSet::<(usize, usize, Direction)>::new();
        loop {
            if coords.contains(&(gi, gj, dir)) {
                return true;
            }

            coords.insert((gi, gj, dir));
            match dir {
                Direction::Up => {
                    if gi == 0 {
                        break;
                    }
                    if grid[gi - 1][gj] == '#' {
                        dir = Direction::Right;
                    } else {
                        gi -= 1;
                    }
                }
                Direction::Down => {
                    if gi == grid.len() - 1 {
                        break;
                    }
                    if grid[gi + 1][gj] == '#' {
                        dir = Direction::Left;
                    } else {
                        gi += 1;
                    }
                }
                Direction::Left => {
                    if gj == 0 {
                        break;
                    }
                    if grid[gi][gj - 1] == '#' {
                        dir = Direction::Up;
                    } else {
                        gj -= 1;
                    }
                }
                Direction::Right => {
                    if gj == grid[gi].len() - 1 {
                        break;
                    }
                    if grid[gi][gj + 1] == '#' {
                        dir = Direction::Down;
                    } else {
                        gj += 1;
                    }
                }
            };
        }

        false
    }
}

impl Day for Day06 {
    type Input = Vec<Vec<char>>;
    fn parse_input(input: &str) -> Self::Input {
        input.lines().map(|s| s.chars().collect()).collect()
    }

    type OP1 = u64;
    fn part_1(grid: Self::Input) -> Self::OP1 {
        let mut gj = 0;
        let mut gi = grid
            .iter()
            .position(|r| match r.into_iter().position(|c| c == &'^') {
                Some(j) => {
                    gj = j;
                    return true;
                }
                None => return false,
            })
            .expect("Failed to find guard");

        let mut dir = Direction::Up;
        let mut coords = HashSet::<(usize, usize)>::new();
        loop {
            coords.insert((gi, gj));
            match dir {
                Direction::Up => {
                    if gi == 0 {
                        break;
                    }
                    if grid[gi - 1][gj] == '#' {
                        dir = Direction::Right;
                    } else {
                        gi -= 1;
                    }
                }
                Direction::Down => {
                    if gi == grid.len() - 1 {
                        break;
                    }
                    if grid[gi + 1][gj] == '#' {
                        dir = Direction::Left;
                    } else {
                        gi += 1;
                    }
                }
                Direction::Left => {
                    if gj == 0 {
                        break;
                    }
                    if grid[gi][gj - 1] == '#' {
                        dir = Direction::Up;
                    } else {
                        gj -= 1;
                    }
                }
                Direction::Right => {
                    if gj == grid[gi].len() - 1 {
                        break;
                    }
                    if grid[gi][gj + 1] == '#' {
                        dir = Direction::Down;
                    } else {
                        gj += 1;
                    }
                }
            };
        }

        coords.len() as u64
    }

    type OP2 = u64;
    fn part_2(grid: Self::Input) -> Self::OP2 {
        let mut res = 0;

        let mut gj = 0;
        let gi = grid
            .iter()
            .position(|r| match r.into_iter().position(|c| c == &'^') {
                Some(j) => {
                    gj = j;
                    return true;
                }
                None => return false,
            })
            .expect("Failed to find guard");

        /* Worst code ever to exist */

        let mut grid = grid.clone();
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '.' {
                    grid[i][j] = '#';

                    if Self::find_loop(&grid, (gi, gj)) {
                        res += 1;
                    }

                    grid[i][j] = '.';
                }
            }
        }

        res
    }
}
