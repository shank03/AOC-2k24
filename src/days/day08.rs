use std::collections::{HashMap, HashSet};

use super::Day;

pub struct Day08;

impl Day for Day08 {
    type Input = (HashMap<char, Vec<(isize, isize)>>, Vec<Vec<char>>);
    fn parse_input(input: &str) -> Self::Input {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

        let mut points = HashMap::<char, Vec<(isize, isize)>>::new();
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '.' {
                    continue;
                }

                match points.get_mut(&grid[i][j]) {
                    Some(v) => v.push((i as isize, j as isize)),
                    None => {
                        points.insert(grid[i][j], vec![(i as isize, j as isize)]);
                    }
                }
            }
        }
        (points, grid)
    }

    type OP1 = u64;
    fn part_1((input, grid): Self::Input) -> Self::OP1 {
        let mut points = HashSet::new();
        let (i_len, j_len) = (grid.len() as isize, grid[0].len() as isize);

        for (_, v) in input {
            for (ix, jx) in v.iter() {
                for (iy, jy) in v.iter() {
                    if ix == iy && jx == jy {
                        continue;
                    }

                    let i = ix - iy;
                    let j = jx - jy;

                    let ax = (ix + i, jx + j);
                    let bx = (iy - i, jy - j);

                    if (ax.0 >= 0 && ax.0 < i_len) && (ax.1 >= 0 && ax.1 < j_len) {
                        points.insert(ax);
                    }
                    if (bx.0 >= 0 && bx.0 < i_len) && (bx.1 >= 0 && bx.1 < j_len) {
                        points.insert(bx);
                    }
                }
            }
        }

        points.len() as u64
    }

    type OP2 = u64;
    fn part_2((input, grid): Self::Input) -> Self::OP2 {
        let mut points = HashSet::new();
        let (i_len, j_len) = (grid.len() as isize, grid[0].len() as isize);

        for (_, v) in input {
            for (ix, jx) in v.iter() {
                for (iy, jy) in v.iter() {
                    if ix == iy && jx == jy {
                        continue;
                    }

                    let i = ix - iy;
                    let j = jx - jy;

                    let mut ax = (ix + i, jx + j);
                    while (ax.0 >= 0 && ax.0 < i_len) && (ax.1 >= 0 && ax.1 < j_len) {
                        points.insert(ax);
                        ax = (ax.0 + i, ax.1 + j);
                    }

                    let mut bx = (iy - i, jy - j);
                    while (bx.0 >= 0 && bx.0 < i_len) && (bx.1 >= 0 && bx.1 < j_len) {
                        points.insert(bx);
                        bx = (bx.0 - i, bx.1 - j);
                    }

                    points.insert((*ix as isize, *jx as isize));
                    points.insert((*iy as isize, *jy as isize));
                }
            }
        }

        points.len() as u64
    }
}
