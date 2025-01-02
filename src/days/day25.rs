use super::Day;

pub struct Day25;

impl Day25 {
    fn overlap(lock: &Vec<i32>, key: &Vec<i32>) -> bool {
        for (l, k) in lock.into_iter().zip(key) {
            if k + l > 5 {
                return false;
            }
        }
        true
    }
}

impl Day for Day25 {
    type Input = (Vec<Vec<i32>>, Vec<Vec<i32>>);
    fn parse_input(input: &str) -> Self::Input {
        let matrices: Vec<Vec<Vec<i32>>> = input
            .split("\n\n")
            .map(|lines| {
                lines
                    .lines()
                    .map(|l| l.chars().map(|c| if c == '#' { -1 } else { 0 }).collect())
                    .collect()
            })
            .collect();

        matrices
            .into_iter()
            .fold((Vec::new(), Vec::new()), |(mut locks, mut keys), matrix| {
                let is_lock = matrix[0][0] == -1;

                let mut v = Vec::new();
                if is_lock {
                    for j in 0..matrix[0].len() {
                        let mut len = 0;
                        let mut i = 1;
                        while i < matrix.len() - 1 && matrix[i][j] == -1 {
                            len += 1;
                            i += 1;
                        }
                        v.push(len);
                    }
                    locks.push(v);
                } else {
                    for j in 0..matrix[0].len() {
                        let mut len = 0;
                        let mut i = matrix.len() - 2;
                        while i > 0 && matrix[i][j] == -1 {
                            len += 1;
                            i -= 1;
                        }
                        v.push(len);
                    }
                    keys.push(v);
                }

                (locks, keys)
            })
    }

    type OP1 = u64;
    fn part_1((locks, keys): Self::Input) -> Self::OP1 {
        let mut res = 0;
        for lock in locks.into_iter() {
            for key in keys.iter() {
                if Self::overlap(&lock, key) {
                    res += 1;
                }
            }
        }
        res
    }

    type OP2 = u64;
    fn part_2(_input: Self::Input) -> Self::OP2 {
        0
    }
}
