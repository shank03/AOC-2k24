use std::collections::{HashMap, VecDeque};

use super::{day06::Direction, Day};

const NUM_KEYPAD: [[Option<char>; 3]; 4] = [
    [Some('7'), Some('8'), Some('9')],
    [Some('4'), Some('5'), Some('6')],
    [Some('1'), Some('2'), Some('3')],
    [None, Some('0'), Some('A')],
];

const ARROW_KEYPAD: [[Option<char>; 3]; 2] = [
    [None, Some('^'), Some('A')],
    [Some('<'), Some('v'), Some('>')],
];

pub struct Day21;

impl Day21 {
    fn map_dir_move(dir: &Direction) -> &str {
        match dir {
            Direction::Up => "^",
            Direction::Down => "v",
            Direction::Left => "<",
            Direction::Right => ">",
        }
    }

    fn find_path(
        key_pad: &[[Option<char>; 3]],
        (is, js): (usize, usize),
        end: char,
    ) -> Vec<String> {
        let directions = vec![
            (0, 1, Direction::Right),
            (-1, 0, Direction::Up),
            (1, 0, Direction::Down),
            (0, -1, Direction::Left),
        ];
        let mut queue = VecDeque::new();
        queue.push_back((is, js, "".to_string()));

        let mut count = usize::MAX;
        let mut paths = Vec::new();
        while let Some((i, j, path)) = queue.pop_front() {
            for (di, dj, dir) in directions.iter() {
                let i = di + i as isize;
                let j = dj + j as isize;

                if i < 0
                    || j < 0
                    || (i as usize) >= key_pad.len()
                    || (j as usize) >= key_pad[0].len()
                {
                    continue;
                }

                let (i, j) = (i as usize, j as usize);
                let c = match key_pad[i][j] {
                    Some(c) => c,
                    None => continue,
                };

                if c == end {
                    if count < path.len() + 1 {
                        return paths;
                    }

                    count = path.len() + 1;
                    paths.push(format!("{path}{}A", Self::map_dir_move(dir)));
                } else {
                    queue.push_back((i, j, format!("{path}{}", Self::map_dir_move(dir))));
                }
            }
        }
        paths
    }

    fn compute_sequence(key_pad: &[[Option<char>; 3]]) -> HashMap<(char, char), Vec<String>> {
        let mut sequence = HashMap::new();

        let key_pos = key_pad
            .into_iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (i, keys)| {
                for (j, c) in keys.into_iter().enumerate() {
                    if let Some(c) = c {
                        acc.insert(*c, (i, j));
                    }
                }
                acc
            });

        for (x, (i, j)) in key_pos.iter() {
            for (y, _) in key_pos.iter() {
                if x == y {
                    sequence.insert((*x, *y), vec!["A".to_string()]);
                    continue;
                }

                sequence.insert((*x, *y), Self::find_path(key_pad, (*i, *j), *y));
            }
        }

        sequence
    }

    fn trace(sequence: &HashMap<(char, char), Vec<String>>, code: &str) -> Vec<String> {
        let mut moves = Vec::new();
        for (x, y) in format!("A{code}").chars().zip(code.chars()) {
            if let Some(v) = sequence.get(&(x, y)) {
                moves.push(v);
            }
        }

        let mut queue = VecDeque::new();
        queue.push_back("".to_string());
        for v in moves {
            let mut strokes = Vec::new();
            while let Some(p) = queue.pop_front() {
                for i in v.iter() {
                    strokes.push(format!("{p}{i}"));
                }
            }
            queue.extend(strokes);
        }

        queue.into_iter().collect()
    }

    fn dp_trace(
        sequence: &HashMap<(char, char), Vec<String>>,
        (x, y): (char, char),
        depth: usize,
        mp: &mut HashMap<(char, char, usize), usize>,
    ) -> usize {
        if depth == 1 {
            let len = sequence.get(&(x, y)).unwrap()[0].len();
            return len;
        }

        if let Some(len) = mp.get(&(x, y, depth)) {
            return *len;
        }

        let mut res = usize::MAX;
        for s in sequence.get(&(x, y)).unwrap().into_iter() {
            let mut len = 0;
            for (a, b) in format!("A{s}").chars().zip(s.chars()) {
                len += Self::dp_trace(sequence, (a, b), depth - 1, mp);
            }
            res = res.min(len);
        }

        mp.insert((x, y, depth), res);
        res
    }

    fn solve(input: Vec<String>, depth: usize) -> usize {
        let num_seq = Self::compute_sequence(&NUM_KEYPAD);
        let arrow_seq = Self::compute_sequence(&ARROW_KEYPAD);

        input.into_iter().fold(0, |acc, code| {
            let mut res = usize::MAX;
            let strokes = Self::trace(&num_seq, &code);
            let mut mp = HashMap::new();

            for seq in strokes {
                let mut len = 0;
                for (a, b) in format!("A{seq}").chars().zip(seq.chars()) {
                    len += Self::dp_trace(&arrow_seq, (a, b), depth, &mut mp);
                }
                res = res.min(len);
            }

            acc + (res * &code[0..3].parse().unwrap())
        })
    }
}

impl Day for Day21 {
    type Input = Vec<String>;
    fn parse_input(input: &str) -> Self::Input {
        input.lines().map(|l| l.to_owned()).collect()
    }

    type OP1 = usize;
    fn part_1(input: Self::Input) -> Self::OP1 {
        Self::solve(input, 2)
    }

    type OP2 = usize;
    fn part_2(input: Self::Input) -> Self::OP2 {
        Self::solve(input, 25)
    }
}
