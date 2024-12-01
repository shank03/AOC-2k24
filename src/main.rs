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
