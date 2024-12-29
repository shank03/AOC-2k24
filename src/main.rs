use clap::{Args, Parser, Subcommand};
use days::Day;
use reqwest::blocking::Client;

mod days;

const YEAR: usize = 2024;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Execute puzzle by day.")]
    Run {
        #[command(flatten)]
        opts: InputDay,
    },
    #[command(about = "Download an input file by day.")]
    GetInput {
        #[command(flatten)]
        opts: InputDay,
    },
}

#[derive(Args)]
#[group(multiple = false)]
struct InputDay {
    #[arg()]
    day: usize,
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Run { opts } => run_input(opts.day),
        Command::GetInput { opts } => download_input(opts.day),
    }
}

fn run_input(day: usize) {
    println!("=== Running DAY {:02} ===", day);

    let file = format!("inputs/day{:02}.txt", day);
    match day {
        1 => days::day01::Day01::run(&file),
        2 => days::day02::Day02::run(&file),
        3 => days::day03::Day03::run(&file),
        4 => days::day04::Day04::run(&file),
        5 => days::day05::Day05::run(&file),
        6 => days::day06::Day06::run(&file),
        7 => days::day07::Day07::run(&file),
        9 => days::day09::Day09::run(&file),
        10 => days::day10::Day10::run(&file),
        11 => days::day11::Day11::run(&file),
        12 => days::day12::Day12::run(&file),
        13 => days::day13::Day13::run(&file),
        14 => days::day14::Day14::run(&file),
        15 => days::day15::Day15::run(&file),
        17 => days::day17::Day17::run(&file),
        18 => days::day18::Day18::run(&file),
        19 => days::day19::Day19::run(&file),
        22 => days::day22::Day22::run(&file),
        23 => days::day23::Day23::run(&file),
        24 => days::day24::Day24::run(&file),
        _ => println!("unknown day"),
    };
}

fn download_input(day: usize) {
    let session = std::fs::read_to_string(".session").expect("Failed to read session token");
    let url = format!("https://adventofcode.com/{}/day/{}/input", YEAR, day);

    let client = Client::new();
    let res = client
        .get(url)
        .header("cookie", format!("session={};", session.trim()))
        .send()
        .expect("Failed to make AOC input request");

    if res.status().is_success() {
        let text = res.text().expect("Failed to read response for AOC input");
        std::fs::write(&format!("inputs/day{:02}.txt", day), text.trim())
            .expect("Failed to save input file");
        println!("Input for day {} saved.", day);
    } else {
        panic!(
            "Request to get input failed - {} - {:?}",
            res.status(),
            res.text()
        );
    }
}
