use std::collections::HashSet;

use super::Day;

pub struct Day20;

impl Day20 {
    fn calc_dists(grid: &Vec<Vec<char>>, (mut is, mut js): (usize, usize)) -> Vec<Vec<i32>> {
        let directions = vec![(0, 1), (-1, 0), (1, 0), (0, -1)];
        let mut dists = vec![vec![-1; grid[0].len()]; grid.len()];
        dists[is][js] = 0;

        while grid[is][js] != 'E' {
            for (di, dj) in directions.iter() {
                let i = di + is as isize;
                let j = dj + js as isize;

                if i < 0 || j < 0 || (i as usize) >= grid.len() || (j as usize) >= grid[0].len() {
                    continue;
                }

                let (i, j) = (i as usize, j as usize);
                if grid[i][j] == '#' {
                    continue;
                }
                if dists[i][j] != -1 {
                    continue;
                }

                dists[i][j] = dists[is][js] + 1;
                is = i;
                js = j;
            }
        }

        dists
    }
}

impl Day for Day20 {
    type Input = (Vec<Vec<char>>, (usize, usize));
    fn parse_input(input: &str) -> Self::Input {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

        let mut s: Option<(usize, usize)> = None;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if s.is_some() {
                    break;
                }
                if grid[i][j] == 'S' {
                    s = Some((i, j));
                }
            }
            if s.is_some() {
                break;
            }
        }
        (grid, s.unwrap())
    }

    type OP1 = usize;
    fn part_1((grid, s): Self::Input) -> Self::OP1 {
        let diagonals = vec![(2, 0), (1, 1), (0, 2), (-1, 1)];
        let dists = Self::calc_dists(&grid, s);

        let mut res = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '#' {
                    continue;
                }

                for (di, dj) in diagonals.iter() {
                    let ni = di + i as isize;
                    let nj = dj + j as isize;

                    if ni < 0
                        || nj < 0
                        || (ni as usize) >= grid.len()
                        || (nj as usize) >= grid[0].len()
                    {
                        continue;
                    }

                    let (ni, nj) = (ni as usize, nj as usize);
                    if grid[ni][nj] == '#' {
                        continue;
                    }

                    let diff = (dists[i][j] as i32).abs_diff(dists[ni][nj]);
                    if diff >= 102 {
                        res += 1;
                    }
                }
            }
        }

        res
    }

    type OP2 = usize;
    fn part_2((grid, s): Self::Input) -> Self::OP2 {
        let dists = Self::calc_dists(&grid, s);

        let mut res = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '#' {
                    continue;
                }

                for rad in 2..21 {
                    for di in 0..(rad + 1) as isize {
                        let dj = rad - di;

                        let mut diagonals = HashSet::new();
                        diagonals.insert((i as isize + di, j as isize + dj));
                        diagonals.insert((i as isize + di, j as isize - dj));
                        diagonals.insert((i as isize - di, j as isize - dj));
                        diagonals.insert((i as isize - di, j as isize + dj));

                        for (ni, nj) in diagonals.into_iter() {
                            if ni < 0
                                || nj < 0
                                || (ni as usize) >= grid.len()
                                || (nj as usize) >= grid[0].len()
                            {
                                continue;
                            }

                            let (ni, nj) = (ni as usize, nj as usize);
                            if grid[ni][nj] == '#' {
                                continue;
                            }

                            if (dists[i][j] - dists[ni][nj]) >= (100 + rad as i32) {
                                res += 1;
                            }
                        }
                    }
                }
            }
        }

        res
    }
}
