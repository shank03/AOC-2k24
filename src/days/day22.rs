use super::Day;

pub struct Day22;

const MODULO: usize = 16777216;

impl Day22 {
    fn next(mut num: usize) -> usize {
        // MUL
        num ^= num * 64;
        num %= MODULO;

        // DIV
        num ^= num / 32;
        num %= MODULO;

        num ^= num * 2048;
        num % MODULO
    }

    fn secret(num: usize) -> usize {
        let mut i = num;
        for _ in 0..2000 {
            i = Self::next(i);
        }
        i
    }

    fn secret_price(prefix: &mut Vec<(u32, u32)>, id: u32, mut num: usize) {
        let mut last;
        let mut hash = 0u32;
        let mut shifts = 0;

        for _ in 0..2000 {
            last = num;
            num = Self::next(num);

            let p = num % 10;
            let diff = p as i8 - (last % 10) as i8 + 9;
            hash = (hash << 8) | (diff as u32 & 0xFF);

            shifts += 1;
            if shifts > 3 {
                let idx = ((hash & 0xFF000000) >> 24) * 19u32.pow(3)
                    + ((hash & 0xFF0000) >> 16) * 19 * 19
                    + ((hash & 0xFF00) >> 8) * 19
                    + (hash & 0xFF);

                let (i, v) = prefix[idx as usize];
                if i != id {
                    prefix[idx as usize] = (id, p as u32 + v);
                }
            }
        }
    }
}

impl Day for Day22 {
    type Input = Vec<usize>;
    fn parse_input(input: &str) -> Self::Input {
        input.lines().map(|l| l.parse().unwrap()).collect()
    }

    type OP1 = usize;
    fn part_1(input: Self::Input) -> Self::OP1 {
        input.into_iter().fold(0, |acc, i| acc + Self::secret(i))
    }

    type OP2 = u32;
    fn part_2(input: Self::Input) -> Self::OP2 {
        let mut prefix = vec![(u32::MAX, 0); 19usize.pow(4)];
        let mut id = 0;

        for num in input {
            Self::secret_price(&mut prefix, id, num);
            id += 1;
        }
        prefix.into_iter().fold(0, |acc, (_, i)| acc.max(i))
    }
}
