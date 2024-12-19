use std::collections::{BTreeMap, HashSet};

use super::Day;

pub struct Day12;

impl Day12 {
    fn traverse(
        garden: &Vec<Vec<char>>,
        (i, j): (usize, usize),
        plant: char,
        visited: &mut Vec<Vec<bool>>,
    ) -> Vec<(usize, usize)> {
        if visited[i][j] {
            return vec![];
        }

        if garden[i][j] != plant {
            return vec![];
        }
        visited[i][j] = true;

        let mut res = vec![(i, j)];

        if i > 0 {
            res.extend(Self::traverse(garden, (i - 1, j), plant, visited));
        }
        if j > 0 {
            res.extend(Self::traverse(garden, (i, j - 1), plant, visited));
        }
        if i < (garden.len() - 1) {
            res.extend(Self::traverse(garden, (i + 1, j), plant, visited));
        }
        if j < (garden[i].len() - 1) {
            res.extend(Self::traverse(garden, (i, j + 1), plant, visited));
        }

        res
    }

    fn get_cost(coords: Vec<(usize, usize)>) -> u64 {
        let mut perimeter = 0;

        let area = coords.len() as u64;
        let (lr_coords, ud_coords) = coords.into_iter().fold(
            (BTreeMap::new(), BTreeMap::new()),
            |(mut lr_acc, mut ud_acc), (y, x)| {
                lr_acc
                    .entry(y)
                    .and_modify(|v: &mut Vec<usize>| v.push(x))
                    .or_insert(vec![x]);
                ud_acc
                    .entry(x)
                    .and_modify(|v: &mut Vec<usize>| v.push(y))
                    .or_insert(vec![y]);
                (lr_acc, ud_acc)
            },
        );

        let mut measure = |coords: BTreeMap<usize, Vec<usize>>| {
            // start border
            perimeter += coords.len() as u64;

            for (_, v) in coords.iter() {
                for i in 1..v.len() {
                    if v[i] - v[i - 1] != 1 {
                        // non-contiguous border
                        perimeter += 2;
                    }
                }
                // end border
                perimeter += 1;
            }
        };

        measure(lr_coords);
        measure(ud_coords);

        perimeter * area
    }

    fn get_discount(coords: Vec<(usize, usize)>) -> u64 {
        let mut perimeter = 0;

        let area = coords.len() as u64;
        let (lr_coords, ud_coords) = coords.into_iter().fold(
            (BTreeMap::new(), BTreeMap::new()),
            |(mut lr_acc, mut ud_acc), (y, x)| {
                lr_acc
                    .entry(y)
                    .and_modify(|v: &mut Vec<isize>| v.push(x as isize))
                    .or_insert(vec![x as isize]);
                ud_acc
                    .entry(x)
                    .and_modify(|v: &mut Vec<isize>| v.push(y as isize))
                    .or_insert(vec![y as isize]);
                (lr_acc, ud_acc)
            },
        );

        let mut measure = |coords: BTreeMap<usize, Vec<isize>>| {
            let mut s_edges = HashSet::new();
            let mut e_edges = HashSet::new();
            for (_, v) in coords.into_iter() {
                let (sp, s) = (v[0] - 1, v[0]);
                let e = *v.last().unwrap();
                let en = e + 1;

                let mut cs = HashSet::new();
                let mut ce = HashSet::new();

                if !s_edges.contains(&(sp, s)) {
                    perimeter += 1;
                }
                cs.insert((sp, s));

                if !e_edges.contains(&(e, en)) {
                    perimeter += 1;
                }
                ce.insert((e, en));

                for w in v.windows(2) {
                    if w[1] - w[0] == 1 {
                        continue;
                    }

                    if !s_edges.contains(&(w[0], w[0] + 1)) && !e_edges.contains(&(w[0], w[0] + 1))
                    {
                        perimeter += 1;
                    }
                    ce.insert((w[0], w[0] + 1));
                    if !s_edges.contains(&(w[1] - 1, w[1])) && !e_edges.contains(&(w[1] - 1, w[1]))
                    {
                        perimeter += 1;
                    }
                    cs.insert((w[1] - 1, w[1]));
                }
                s_edges = cs;
                e_edges = ce;
            }
        };

        measure(lr_coords);
        measure(ud_coords);

        perimeter * area
    }
}

impl Day for Day12 {
    type Input = Vec<Vec<char>>;
    fn parse_input(input: &str) -> Self::Input {
        input.lines().map(|s| s.chars().collect()).collect()
    }

    type OP1 = u64;
    fn part_1(garden: Self::Input) -> Self::OP1 {
        let mut res = 0;

        let mut visited = vec![vec![false; garden[0].len()]; garden.len()];
        for i in 0..garden.len() {
            for j in 0..garden[i].len() {
                if visited[i][j] {
                    continue;
                }

                let mut coords = Self::traverse(&garden, (i, j), garden[i][j], &mut visited);
                coords.sort();

                res += Self::get_cost(coords);
            }
        }

        res
    }

    type OP2 = u64;
    fn part_2(garden: Self::Input) -> Self::OP2 {
        let mut res = 0;

        let mut visited = vec![vec![false; garden[0].len()]; garden.len()];

        for i in 0..garden.len() {
            for j in 0..garden[i].len() {
                if visited[i][j] {
                    continue;
                }

                let mut coords = Self::traverse(&garden, (i, j), garden[i][j], &mut visited);
                coords.sort();

                res += Self::get_discount(coords);
            }
        }

        res
    }
}
