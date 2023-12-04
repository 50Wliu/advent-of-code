pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;

pub mod utilities;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    // Day to run
    #[arg(short='d', long, default_value_t = 0)]
    day: usize,

    // Part to run
    #[arg(short='p', long, default_value_t = 0)]
    part: usize,
}

fn main() {
    let args = Cli::parse();

    let puzzles: Vec<Vec<fn() -> u32>> = Vec::from([
        Vec::from([day_01::part_1, day_01::part_2]),
        Vec::from([day_02::part_1, day_02::part_2]),
        Vec::from([day_03::part_1, day_03::part_2]),
        Vec::from([day_04::part_1, day_04::part_2]),
    ]);

    let mut days_to_run = 1..=puzzles.len();
    let mut parts_to_run = 1..=2;

    if args.day > puzzles.len() {
        panic!("day is {} not yet registered in main.rs", args.day);
    }
    if args.part > 2 {
        panic!("part {} does not exist", args.part);
    }

    if args.day != 0 {
        days_to_run = args.day..=args.day;
    }
    if args.part != 0 {
        parts_to_run = args.part..=args.part;
    }

    for day in days_to_run {
        for part in parts_to_run.clone() {
            let func = puzzles[day - 1][part - 1];
            println!("{}", func());
        }
    }
}            

