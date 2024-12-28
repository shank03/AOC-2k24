use std::collections::{HashMap, HashSet};

use super::Day;

#[derive(Debug, Clone, Copy)]
pub enum Op {
    AND,
    OR,
    XOR,
}
impl Op {
    fn exec(&self, x: u8, y: u8) -> u8 {
        match self {
            Op::AND => x & y,
            Op::OR => x | y,
            Op::XOR => x ^ y,
        }
    }
}
impl From<&str> for Op {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Self::AND,
            "OR" => Self::OR,
            "XOR" => Self::XOR,
            _ => unimplemented!(),
        }
    }
}

pub struct Day24;

impl Day24 {
    fn equate(
        var: &String,
        graph: &HashMap<String, (String, Op, String)>,
        values: &mut HashMap<String, u8>,
    ) -> u8 {
        if let Some(val) = values.get(var) {
            return *val;
        }

        let (x, op, y) = graph.get(var).unwrap();
        let x = values
            .get(x)
            .map(|u| *u)
            .unwrap_or(Self::equate(x, graph, values));
        let y = values
            .get(y)
            .map(|u| *u)
            .unwrap_or(Self::equate(y, graph, values));

        op.exec(x, y)
    }
}

impl Day for Day24 {
    type Input = (HashMap<String, u8>, HashMap<String, (String, Op, String)>);
    fn parse_input(input: &str) -> Self::Input {
        let mut parts = input.split("\n\n");

        let initials = parts.next().unwrap();
        let initials = initials.lines().fold(HashMap::new(), |mut acc, l| {
            let mut toks = l.split(':');
            let var = toks.next().unwrap().trim();
            let b = toks.next().unwrap().trim().parse::<u8>().unwrap();
            match acc.get_mut(var) {
                None => {
                    acc.insert(var.to_owned(), b);
                }
                _ => (),
            };
            acc
        });

        let edges = parts.next().unwrap();
        let graph = edges.lines().fold(HashMap::new(), |mut acc, l| {
            let mut toks = l.split(' ');
            let x = toks.next().unwrap().trim();
            let op = toks.next().unwrap().trim();
            let y = toks.next().unwrap().trim();

            let res = toks.last().unwrap().trim();
            match acc.get(res) {
                None => {
                    acc.insert(res.to_owned(), (x.to_owned(), Op::from(op), y.to_owned()));
                }
                _ => (),
            };
            acc
        });
        (initials, graph)
    }

    type OP1 = u64;
    fn part_1((mut values, graph): Self::Input) -> Self::OP1 {
        for (var, _) in graph.iter() {
            let val = Self::equate(var, &graph, &mut values);
            values.insert(var.to_owned(), val);
        }
        let mut values = values
            .into_iter()
            .filter(|(k, _)| k.starts_with('z'))
            .collect::<Vec<_>>();
        values.sort_by(|(a, _), (b, _)| a.cmp(b).reverse());
        values
            .into_iter()
            .fold(0, |acc, (_, i)| (acc << 1) | (i as u64))
    }

    type OP2 = String;
    fn part_2((_, graph): Self::Input) -> Self::OP2 {
        let mut swaps = HashSet::new();
        let zm = graph
            .iter()
            .fold("z00".to_owned(), |acc, (k, _)| acc.max(k.to_owned()));
        let xyz = ['x', 'y', 'z'];

        for (var, (x, op, y)) in graph.iter() {
            if var.starts_with('z') && !matches!(op, Op::XOR) && var != &zm {
                swaps.insert(var);
            }
            if matches!(op, Op::XOR)
                && !xyz
                    .iter()
                    .any(|c| var.starts_with(*c) || x.starts_with(*c) || y.starts_with(*c))
            {
                swaps.insert(var);
            }
            if matches!(op, Op::AND) && x != "x00" && y != "x00" {
                for (_, (sx, sop, sy)) in graph.iter() {
                    if (var == sx || var == sy) && !matches!(sop, Op::OR) {
                        swaps.insert(var);
                    }
                }
            }
            if matches!(op, Op::XOR) {
                for (_, (sx, sop, sy)) in graph.iter() {
                    if (var == sx || var == sy) && matches!(sop, Op::OR) {
                        swaps.insert(var);
                    }
                }
            }
        }
        let mut swaps = swaps.into_iter().map(|s| s.to_owned()).collect::<Vec<_>>();
        swaps.sort();
        swaps.join(",")
    }
}
