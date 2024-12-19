use std::{collections::HashMap, isize};

use super::Day;

#[derive(Debug)]
pub struct Claw {
    a: (isize, isize),
    b: (isize, isize),
    prize: (isize, isize),
}

pub struct Day13;

impl Day13 {
    fn dp(claw: &Claw, (rx, ry): (isize, isize), tb: &mut HashMap<(isize, isize), isize>) -> isize {
        if rx > claw.prize.0 || ry > claw.prize.1 {
            return isize::MAX;
        }

        if let Some(res) = tb.get(&(rx, ry)) {
            return *res;
        }

        let cx = rx + claw.a.0;
        let cy = ry + claw.a.1;
        let l = if cx == claw.prize.0 && cy == claw.prize.1 {
            3
        } else {
            let v = Self::dp(claw, (cx, cy), tb);
            if v != isize::MAX {
                3 + v
            } else {
                v
            }
        };

        let cx = rx + claw.b.0;
        let cy = ry + claw.b.1;
        let r = if cx == claw.prize.0 && cy == claw.prize.1 {
            1
        } else {
            let v = Self::dp(claw, (cx, cy), tb);
            if v != isize::MAX {
                1 + v
            } else {
                v
            }
        };

        let res = match (l, r) {
            (isize::MAX, isize::MAX) => isize::MAX,
            (isize::MAX, _) => r,
            (_, isize::MAX) => l,
            (_, _) => l.min(r),
        };

        tb.insert((rx, ry), res);
        res
    }
}

impl Day for Day13 {
    type Input = Vec<Claw>;
    fn parse_input(input: &str) -> Self::Input {
        input
            .split("\n\n")
            .map(|s| {
                let mut l = s.lines();
                let a = l.next().unwrap();
                let b = l.next().unwrap();
                let prize = l.next().unwrap();

                let mut a = a.split(',');
                let ax = a.next().unwrap().split('+').last().unwrap();
                let ay = a.next().unwrap().split('+').last().unwrap();

                let mut b = b.split(',');
                let bx = b.next().unwrap().split('+').last().unwrap();
                let by = b.next().unwrap().split('+').last().unwrap();

                let mut p = prize.split(',');
                let px = p.next().unwrap().split('=').last().unwrap();
                let py = p.next().unwrap().split('=').last().unwrap();

                Claw {
                    a: (ax.parse().unwrap(), ay.parse().unwrap()),
                    b: (bx.parse().unwrap(), by.parse().unwrap()),
                    prize: (px.parse().unwrap(), py.parse().unwrap()),
                }
            })
            .collect()
    }

    type OP1 = isize;
    fn part_1(input: Self::Input) -> Self::OP1 {
        input.into_iter().fold(0, |acc, claw| {
            let mut tb = HashMap::new();
            let r = Self::dp(&claw, (0, 0), &mut tb);
            acc + if r == isize::MAX { 0 } else { r }
        })
    }

    type OP2 = isize;
    fn part_2(input: Self::Input) -> Self::OP2 {
        let valid = |a: (isize, isize), b: (isize, isize)| -> bool {
            (a.0 / a.1) == (b.0 / b.1) && ((a.0 % a.1) * b.1) == ((b.0 % b.1) * a.1)
        };

        input.into_iter().fold(0, |acc, claw| {
            let claw = Claw {
                a: claw.a,
                b: claw.b,
                prize: (claw.prize.0 + 10000000000000, claw.prize.1 + 10000000000000),
            };

            if valid(claw.prize, claw.a) {
                if valid(claw.a, claw.b) {
                    return acc;
                }
            } else if valid(claw.prize, claw.b) {
                return acc
                    + if claw.prize.0 % claw.b.0 == 0 {
                        claw.prize.0 / claw.b.0
                    } else {
                        0
                    };
            } else if valid(claw.a, claw.b) {
                return acc;
            }

            let n = (claw.prize.1 * claw.a.0) - (claw.prize.0 * claw.a.1);
            let d = (claw.b.1 * claw.a.0) - (claw.b.0 * claw.a.1);

            if d != 0 && n % d == 0 {
                let b = n / d;
                let n = claw.prize.0 - (b * claw.b.0);
                if claw.a.0 != 0 && n % claw.a.0 == 0 {
                    return acc + b + (3 * (n / claw.a.0));
                }
            }

            if claw.a.0 == 0 && claw.prize.0 % claw.b.0 == 0 {
                let b = claw.prize.0 / claw.b.0;
                let n = claw.prize.1 - (b * claw.b.1);
                if claw.a.1 != 0 && n % claw.a.1 == 0 {
                    return acc + b + (3 * (n / claw.a.1));
                }
            }

            acc
        })
    }
}
