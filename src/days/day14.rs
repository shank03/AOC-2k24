use super::Day;

#[derive(Debug, Clone)]
pub struct Bot {
    pos: (isize, isize),
    dir: (isize, isize),
}

pub struct Day14;

impl Day for Day14 {
    type Input = Vec<Bot>;
    fn parse_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(|s| {
                let mut p = s.split(' ');
                let pos = p.next().unwrap().split('=').last().unwrap();
                let dir = p.next().unwrap().split('=').last().unwrap();

                let mut pos = pos.split(',');
                let mut dir = dir.split(',');

                let pos = (
                    pos.next().unwrap().parse().unwrap(),
                    pos.last().unwrap().parse().unwrap(),
                );
                let dir = (
                    dir.next().unwrap().parse().unwrap(),
                    dir.last().unwrap().parse().unwrap(),
                );
                Bot {
                    pos: (pos.1, pos.0),
                    dir: (dir.1, dir.0),
                }
            })
            .collect()
    }

    type OP1 = u64;
    fn part_1(input: &Self::Input) -> Self::OP1 {
        let (i, j) = input
            .into_iter()
            .fold((0, 0), |(i_a, j_a), b| (i_a.max(b.pos.0), j_a.max(b.pos.1)));
        let (i, j) = (i + 1, j + 1);
        let (mid_i, mid_j) = (i / 2, j / 2);

        let cc = |pos: isize, dir: isize, m: isize|  if pos + dir < 0 {
            m + (pos + dir)
        } else {
            pos + dir
        } % m;

        let bots: Vec<(isize, isize)> = input
            .into_iter()
            .map(|bot| {
                let mut pos = bot.pos;
                for _ in 0..100 {
                    pos = (cc(pos.0, bot.dir.0, i), cc(pos.1, bot.dir.1, j));
                }
                pos
            })
            .collect();

        let mut quads = [0; 4];
        for (i, j) in bots.into_iter() {
            if i == mid_i || j == mid_j {
                continue;
            }

            if i > mid_i {
                if j > mid_j {
                    quads[1] += 1;
                } else {
                    quads[2] += 1;
                }
            } else {
                if j > mid_j {
                    quads[0] += 1;
                } else {
                    quads[3] += 1;
                }
            }
        }

        quads.into_iter().fold(1, |acc, i| acc * i)
    }

    type OP2 = u64;
    fn part_2(input: &Self::Input) -> Self::OP2 {
        let mut input: Vec<_> = input.into_iter().cloned().collect();

        let (i, j) = input
            .iter()
            .fold((0, 0), |(i_a, j_a), b| (i_a.max(b.pos.0), j_a.max(b.pos.1)));
        let (i, j) = (i + 1, j + 1);

        let cc = |pos: isize, dir: isize, m: isize|  if pos + dir < 0 {
            m + (pos + dir)
        } else {
            pos + dir
        } % m;

        for s in 1..10_000 {
            let mut bots: Vec<(isize, isize)> = Vec::new();
            input.iter_mut().for_each(|bot| {
                bot.pos = (cc(bot.pos.0, bot.dir.0, i), cc(bot.pos.1, bot.dir.1, j));
                bots.push(bot.pos);
            });

            let mut grid = vec![vec![-1; j as usize]; i as usize];
            for (y, x) in bots {
                grid[y as usize][x as usize] = 1;
            }

            println!("Step: {s}");

            for i in grid {
                for j in i {
                    print!("{}", if j == -1 { " " } else { "#" });
                }
                println!();
            }
        }

        let quads = [0; 4];
        quads.into_iter().fold(1, |acc, i| acc * i)
    }
}
