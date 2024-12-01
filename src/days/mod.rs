use std::{fmt::Display, fs};

pub mod day01;

pub trait Day {
    type Input;
    fn parse_input(input: &str) -> Self::Input;

    type OP1: Display;
    fn part_1(input: &Self::Input) -> Self::OP1;

    type OP2: Display;
    fn part_2(input: &Self::Input) -> Self::OP2;

    fn read_input(file: &str) -> Self::Input {
        let input_string =
            fs::read_to_string(file).expect(&format!("Failed to read input file - {file}"));
        Self::parse_input(&input_string)
    }

    fn run(input_file: &str) {
        let input = Self::read_input(input_file);
        let start = std::time::Instant::now();
        println!("P1: {} - {:?}", Self::part_1(&input), start.elapsed());

        let start = std::time::Instant::now();
        println!("P2: {} - {:?}", Self::part_2(&input), start.elapsed());
    }
}
