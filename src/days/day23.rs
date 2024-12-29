use std::collections::{BTreeMap, BTreeSet};

use super::Day;

pub struct Day23;

impl Day for Day23 {
    type Input = BTreeMap<String, Vec<String>>;
    fn parse_input(input: &str) -> Self::Input {
        input.lines().fold(BTreeMap::new(), |mut acc, l| {
            let mut l = l.split('-');
            let x = l.next().unwrap().trim().to_owned();
            let y = l.next().unwrap().to_owned();

            match acc.get_mut(&x) {
                Some(e) => {
                    e.push(y.clone());
                    e.sort();
                }
                None => {
                    acc.insert(x.clone(), vec![y.clone()]);
                }
            };
            match acc.get_mut(&y) {
                Some(e) => {
                    e.push(x);
                    e.sort();
                }
                None => {
                    acc.insert(y, vec![x]);
                }
            };
            acc
        })
    }

    type OP1 = u64;
    fn part_1(graph: Self::Input) -> Self::OP1 {
        let mut sets = BTreeSet::<BTreeSet<&String>>::new();

        for (node, nodes) in graph.iter() {
            for l1 in nodes {
                for l2 in graph.get(l1).unwrap() {
                    if graph.get(l2).unwrap().iter().any(|c| c == node) {
                        sets.insert(BTreeSet::from([node, l1, l2]));
                    }
                }
            }
        }

        sets.into_iter().fold(0, |acc, s| {
            acc + if s.into_iter().any(|c| c.starts_with('t')) {
                1
            } else {
                0
            }
        })
    }

    type OP2 = String;
    fn part_2(graph: Self::Input) -> Self::OP2 {
        let mut vis = BTreeSet::new();
        let mut password = String::new();

        for (node, conn) in graph.iter() {
            for comp in conn.into_iter() {
                let mut edges = vec![node.to_owned(), comp.to_owned()];

                for other in conn.into_iter().filter(|c| c != &comp) {
                    let other_conn = graph.get(other).unwrap();
                    if edges.iter().all(|e| other_conn.contains(e)) {
                        edges.push(other.to_owned());
                    }
                }

                edges.sort();
                let edges = edges.join(",");
                if vis.insert(edges.clone()) && edges.len() > password.len() {
                    password = edges;
                }
            }
        }
        password
    }
}
