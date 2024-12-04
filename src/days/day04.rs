use super::Day;

pub struct Day04;

impl Day for Day04 {
    type Input = Vec<Vec<char>>;
    fn parse_input(input: &str) -> Self::Input {
        input.lines().map(|s| s.chars().collect()).collect()
    }

    type OP1 = u64;
    fn part_1(input: &Self::Input) -> Self::OP1 {
        let mut res = 0;
        let mut lines = Vec::<String>::new();

        // horizontal
        for line in input.iter() {
            lines.push(line.iter().collect());
        }

        // vertical
        for j in 0..input[0].len() {
            let mut s = String::new();
            for line in input.iter() {
                s.push(line[j]);
            }
            lines.push(s);
        }

        // diagonal
        for j in 0..input[0].len() {
            let mut s = String::new();
            let (mut x, mut y) = (0, j);
            while x < input.len() && y < input[0].len() {
                s.push(input[x][y]);
                x += 1;
                y += 1;
            }
            if s.len() < 4 {
                continue;
            }
            lines.push(s);
        }
        for i in 1..input.len() {
            let mut s = String::new();
            let (mut x, mut y) = (i, 0);
            while x < input.len() && y < input[0].len() {
                s.push(input[x][y]);
                x += 1;
                y += 1;
            }
            if s.len() < 4 {
                continue;
            }
            lines.push(s);
        }

        for j in (0..input[0].len()).rev() {
            let mut s = String::new();
            let (mut x, mut y) = (0, j);
            loop {
                s.push(input[x][y]);
                if x == input[0].len() || y == 0 {
                    break;
                }
                x += 1;
                y -= 1;
            }
            if s.len() < 4 {
                continue;
            }
            lines.push(s);
        }
        for i in 1..input.len() {
            let mut s = String::new();
            let (mut x, mut y) = (i, input[0].len() - 1);
            while x < input.len() {
                s.push(input[x][y]);
                if y == 0 {
                    break;
                }
                x += 1;
                y -= 1;
            }
            if s.len() < 4 {
                continue;
            }
            lines.push(s);
        }

        for line in lines {
            res += regex::Regex::new(r"XMAS")
                .unwrap()
                .captures_iter(&line)
                .count() as u64;
            res += regex::Regex::new(r"SAMX")
                .unwrap()
                .captures_iter(&line)
                .count() as u64;
        }

        res
    }

    type OP2 = u64;
    fn part_2(input: &Self::Input) -> Self::OP2 {
        let mut res = 0;

        for i in 1..(input.len() - 1) {
            for j in 1..(input[i].len() - 1) {
                if input[i][j] != 'A' {
                    continue;
                }

                // the two above and below 'A' are equal
                if input[i - 1][j - 1] == input[i - 1][j + 1]
                    && input[i + 1][j - 1] == input[i + 1][j + 1]
                {
                    // two sides are 'S' and 'M' or vice-versa
                    if (input[i - 1][j - 1] == 'S' && input[i + 1][j - 1] == 'M')
                        || (input[i - 1][j - 1] == 'M' && input[i + 1][j - 1] == 'S')
                    {
                        res += 1;
                        continue;
                    }
                }

                // the two left and right 'A' are equal
                if input[i - 1][j - 1] == input[i + 1][j - 1]
                    && input[i - 1][j + 1] == input[i + 1][j + 1]
                {
                    // two sides are 'S' and 'M' or vice-versa
                    if (input[i - 1][j - 1] == 'S' && input[i - 1][j + 1] == 'M')
                        || (input[i - 1][j - 1] == 'M' && input[i - 1][j + 1] == 'S')
                    {
                        res += 1;
                        continue;
                    }
                }
            }
        }

        res
    }
}
