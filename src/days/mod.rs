use std::{fmt::Debug, fs};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day17;

pub trait Day {
    type Input;
    fn parse_input(input: &str) -> Self::Input;

    type OP1: Debug;
    fn part_1(input: Self::Input) -> Self::OP1;

    type OP2: Debug;
    fn part_2(input: Self::Input) -> Self::OP2;

    fn read_input(file: &str) -> Self::Input {
        let input_string =
            fs::read_to_string(file).expect(&format!("Failed to read input file - {file}"));
        Self::parse_input(&input_string)
    }

    fn run(input_file: &str) {
        let input = Self::read_input(input_file);
        let start = std::time::Instant::now();
        println!("P1: {:?} - {:?}", Self::part_1(input), start.elapsed());

        let input = Self::read_input(input_file);
        let start = std::time::Instant::now();
        println!("P2: {:?} - {:?}", Self::part_2(input), start.elapsed());
    }
}
