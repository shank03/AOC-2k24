use std::collections::{BTreeMap, HashSet};

use super::Day;

pub struct Day05;

impl Day for Day05 {
    type Input = (
        BTreeMap<u64, Vec<u64>>,
        BTreeMap<u64, Vec<u64>>,
        Vec<Vec<u64>>,
    );
    fn parse_input(input: &str) -> Self::Input {
        let lines: Vec<&str> = input.lines().collect();
        let offset = lines.iter().position(|s| s.is_empty()).unwrap_or(0);

        let (order, query) = lines.split_at(offset);

        let mut ordered = BTreeMap::<u64, Vec<u64>>::new();
        let mut rev_ordered = BTreeMap::<u64, Vec<u64>>::new();
        for line in order.into_iter() {
            let mut s = line.split('|');
            let a = s.next().unwrap().parse().unwrap_or(0);
            let b = s.last().unwrap().parse().unwrap_or(0);

            match ordered.get_mut(&a) {
                Some(list) => list.push(b),
                None => {
                    ordered.insert(a, vec![b]);
                }
            };

            match rev_ordered.get_mut(&b) {
                Some(list) => list.push(a),
                None => {
                    rev_ordered.insert(b, vec![a]);
                }
            };
        }
        ordered.iter_mut().for_each(|(_, v)| v.sort());
        rev_ordered.iter_mut().for_each(|(_, v)| v.sort());

        let mut queries = Vec::new();
        for line in query.into_iter().filter(|s| !s.is_empty()) {
            queries.push(line.split(',').map(|s| s.parse().unwrap_or(0)).collect());
        }

        (ordered, rev_ordered, queries)
    }

    type OP1 = u64;
    fn part_1((map, _, queries): Self::Input) -> Self::OP1 {
        let mut res = 0;

        for query in queries.into_iter() {
            let mut valid_arr = true;
            let len = query.len();
            for (i, item) in query.iter().enumerate() {
                if let Some(list) = map.get(item) {
                    let mut valid = 0;
                    let mut j = i + 1;
                    while j < len {
                        valid += if let Ok(_) = list.binary_search(&query[j]) {
                            1
                        } else {
                            0
                        };
                        j += 1;
                    }
                    if valid != (j - i - 1) {
                        valid_arr = false;
                        break;
                    }
                } else if i != len - 1 {
                    valid_arr = false;
                    break;
                }
            }

            if !valid_arr {
                continue;
            }

            let mid = if len % 2 == 0 { len / 2 } else { (len + 1) / 2 };
            res += query[mid - 1];
        }

        res
    }

    type OP2 = u64;
    fn part_2((ordered, rev_ordered, queries): Self::Input) -> Self::OP2 {
        let mut res = 0;

        for query in queries.into_iter().filter(|q| {
            let mut na = HashSet::<u64>::new();
            for i in q.into_iter() {
                if na.contains(i) {
                    return true;
                }

                if let Some(k) = rev_ordered.get(i) {
                    k.into_iter().for_each(|x| {
                        let _ = na.insert(*x);
                    });
                }
            }
            return false;
        }) {
            let mut query = query.clone();
            query.sort_by(|a, b| {
                if let Some(list) = ordered.get(a) {
                    if list.binary_search(b).is_ok() {
                        return std::cmp::Ordering::Less;
                    }
                }

                if let Some(list) = ordered.get(b) {
                    if list.binary_search(a).is_ok() {
                        return std::cmp::Ordering::Greater;
                    }
                }

                return std::cmp::Ordering::Equal;
            });

            res += query[(query.len() - 1) / 2]
        }

        res
    }
}
