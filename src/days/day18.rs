use std::collections::VecDeque;

use super::Day;

pub struct Day18;

impl Day18 {
    fn find_path(grid: &Vec<Vec<i32>>, (is, js): (usize, usize), (ie, je): (usize, usize)) -> i64 {
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut queue = VecDeque::new();
        let mut vis = vec![vec![false; grid[0].len()]; grid.len()];

        queue.push_back((is, js, 0));
        vis[is][js] = true;

        while let Some((y, x, steps)) = queue.pop_front() {
            if y == ie && x == je {
                return steps;
            }

            for (dx, dy) in directions.iter() {
                let x = dx + x as isize;
                let y = dy + y as isize;

                if x < 0 || y < 0 || (x as usize) >= grid[0].len() || (y as usize) >= grid.len() {
                    continue;
                }

                let (x, y) = (x as usize, y as usize);
                if grid[y][x] == -1 || vis[y][x] {
                    continue;
                }

                vis[y][x] = true;
                queue.push_back((y, x, steps + 1));
            }
        }

        -1
    }
}

impl Day for Day18 {
    type Input = Vec<(usize, usize)>;
    fn parse_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(|s| {
                let mut l = s.split(',');
                let a = l.next().unwrap().parse().unwrap();
                let b = l.next().unwrap().parse().unwrap();
                (a, b)
            })
            .collect()
    }

    type OP1 = i64;
    fn part_1(coords: Self::Input) -> Self::OP1 {
        let mut grid = vec![vec![0; 71]; 71];
        coords
            .into_iter()
            .take(1024)
            .for_each(|(j, i)| grid[i][j] = -1);

        Self::find_path(&grid, (0, 0), (70, 70))
    }

    type OP2 = (usize, usize);
    fn part_2(coords: Self::Input) -> Self::OP2 {
        let mut grid = vec![vec![0; 71]; 71];
        let (mut y, mut x, mut i) = (0, 0, 0);
        loop {
            if i == coords.len() {
                break;
            }

            let (ib, jb) = coords[i];
            grid[ib][jb] = -1;

            if Self::find_path(&grid, (0, 0), (70, 70)) == -1 {
                (y, x) = (ib, jb);
                break;
            }

            i += 1;
        }

        (y, x)
    }
}
