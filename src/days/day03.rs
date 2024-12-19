use super::Day;

pub struct Day03;

impl Day for Day03 {
    type Input = String;
    fn parse_input(input: &str) -> Self::Input {
        input.to_string()
    }

    type OP1 = u64;
    fn part_1(input: Self::Input) -> Self::OP1 {
        let re = regex::Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
        let mut res = 0;
        for cap in re.captures_iter(&input) {
            for mul in cap.iter() {
                if let Some(mul) = mul {
                    let bracket = mul.as_str().split('(').last().unwrap();
                    let mut nums = bracket.split(',');
                    let a: u64 = nums.next().unwrap().parse().unwrap();
                    let b: u64 = nums.next().unwrap().trim_end_matches(')').parse().unwrap();
                    res += a * b;
                }
            }
        }

        res
    }

    type OP2 = u64;
    fn part_2(input: Self::Input) -> Self::OP2 {
        let mut res = 0;

        let re = regex::Regex::new(r"do\(\)|don't\(\)|mul\([0-9]+,[0-9]+\)").unwrap();
        let mut enabled = true;
        for cap in re.captures_iter(&input) {
            for mul in cap.iter() {
                if let Some(mul) = mul {
                    match mul.as_str() {
                        "don't()" => enabled = false,
                        "do()" => enabled = true,
                        _ => {
                            if enabled {
                                let bracket = mul.as_str().split('(').last().unwrap();
                                let mut nums = bracket.split(',');
                                let a: u64 = nums.next().unwrap().parse().unwrap();
                                let b: u64 =
                                    nums.next().unwrap().trim_end_matches(')').parse().unwrap();
                                res += a * b;
                            }
                        }
                    };
                }
            }
        }

        res
    }
}
