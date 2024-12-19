use std::collections::HashMap;

use super::Day;

pub struct Day01;

impl Day for Day01 {
    type Input = (Vec<u64>, Vec<u64>);

    fn parse_input(input: &str) -> Self::Input {
        let mut start = Vec::<u64>::new();
        let mut end = Vec::<u64>::new();

        input.lines().for_each(|l| {
            let mut nums = l.split(' ');
            start.push(
                nums.next()
                    .expect("Expected first index")
                    .parse()
                    .expect("Expected valid number"),
            );
            end.push(
                nums.last()
                    .expect("Expected second index")
                    .parse()
                    .expect("Expected valid number"),
            );
        });

        start.sort();
        end.sort();

        (start, end)
    }

    type OP1 = u64;

    fn part_1(input: Self::Input) -> Self::OP1 {
        let (st, dst) = input;

        st.into_iter()
            .zip(dst)
            .fold(0, |acc, (a, b)| acc + a.abs_diff(b))
    }

    type OP2 = u64;

    fn part_2(input: Self::Input) -> Self::OP2 {
        let (st, dst) = input;

        let mut map = HashMap::<u64, u64>::new();
        dst.into_iter().for_each(|b| match map.get_mut(&b) {
            Some(v) => *v += 1,
            None => map.insert(b, 1).map(|_| ()).unwrap_or(()),
        });

        st.into_iter()
            .fold(0, |acc, a| acc + (a * map.get(&a).unwrap_or(&0)))
    }
}
