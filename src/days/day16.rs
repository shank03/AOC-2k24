use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use super::{day06::Direction, Day};

pub struct Day16;

#[derive(Debug, PartialEq, Eq)]
pub struct State {
    i: usize,
    j: usize,
    steps: u64,
    direction: Direction,
    points: Vec<(usize, usize)>,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.steps.cmp(&self.steps)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.steps.cmp(&other.steps))
    }
}

impl Day16 {
    fn find_path(
        grid: &Vec<Vec<i32>>,
        (is, js): (usize, usize),
        (ie, je): (usize, usize),
    ) -> Vec<(u64, Vec<(usize, usize)>)> {
        let directions = vec![
            (0, 1, Direction::Right),
            (-1, 0, Direction::Up),
            (1, 0, Direction::Down),
            (0, -1, Direction::Left),
        ];
        let mut queue = BinaryHeap::new();
        let mut vis = HashMap::new();

        queue.push(Reverse(State {
            i: is,
            j: js,
            steps: 0,
            direction: Direction::Right,
            points: vec![(is, js)],
        }));

        let mut min_steps = u64::MAX;
        let mut res = Vec::new();
        while let Some(Reverse(State {
            i,
            j,
            steps,
            direction,
            points,
        })) = queue.pop()
        {
            if steps > min_steps {
                continue;
            }

            if i == ie && j == je {
                if steps < min_steps {
                    min_steps = steps;
                    res.clear();
                }
                res.push((steps, points.clone()));
                continue;
            }

            if vis.get(&(i, j, direction)).map_or(false, |&ps| steps > ps) {
                continue;
            }
            vis.insert((i, j, direction), steps);

            for (di, dj, dr) in directions.iter() {
                let i = di + i as isize;
                let j = dj + j as isize;

                if i < 0 || j < 0 || (i as usize) >= grid.len() || (j as usize) >= grid[0].len() {
                    continue;
                }

                let (i, j) = (i as usize, j as usize);
                if grid[i][j] == -1 {
                    continue;
                }

                let mut points = points.clone();
                points.push((i, j));
                queue.push(Reverse(State {
                    i,
                    j,
                    steps: steps + if dr == &direction { 1 } else { 1001 },
                    direction: *dr,
                    points,
                }));
            }
        }
        res
    }
}

impl Day for Day16 {
    type Input = Vec<Vec<i32>>;
    fn parse_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '#' => -1,
                        'E' => 2,
                        'S' => 1,
                        _ => 0,
                    })
                    .collect()
            })
            .collect()
    }

    type OP1 = u64;
    fn part_1(mut grid: Self::Input) -> Self::OP1 {
        let (mut is, mut js) = (0, 0);
        let (mut ie, mut je) = (0, 0);
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 2 {
                    (ie, je) = (i, j);
                    grid[i][j] = 0;
                }
                if grid[i][j] == 1 {
                    (is, js) = (i, j);
                    grid[i][j] = 0;
                }
            }
        }

        Self::find_path(&grid, (is, js), (ie, je))[0].0
    }

    type OP2 = u64;
    fn part_2(mut grid: Self::Input) -> Self::OP2 {
        let (mut is, mut js) = (0, 0);
        let (mut ie, mut je) = (0, 0);
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 2 {
                    (ie, je) = (i, j);
                    grid[i][j] = 0;
                }
                if grid[i][j] == 1 {
                    (is, js) = (i, j);
                    grid[i][j] = 0;
                }
            }
        }

        Self::find_path(&grid, (is, js), (ie, je))
            .into_iter()
            .fold(HashSet::new(), |mut acc, (_, v)| {
                v.into_iter().for_each(|(i, j)| {
                    acc.insert((i, j));
                });
                acc
            })
            .len() as u64
    }
}
