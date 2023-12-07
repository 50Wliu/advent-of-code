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

fn main() -> Result<(), String> {
    let args = Cli::parse();

    let puzzles: Vec<Vec<fn(&String) -> Result<u32, String>>> = vec![
        vec![day_01::part_1, day_01::part_2],
        vec![day_02::part_1, day_02::part_2],
        vec![day_03::part_1, day_03::part_2],
        vec![day_04::part_1, day_04::part_2],
    ];

    if args.day > puzzles.len() {
        return Err(format!("day is {} not yet registered in main.rs", args.day))

    }
    if args.part > 2 {
        return Err(format!("part {} does not exist", args.part))
    }

    let days_to_run = if args.day != 0 {
        args.day..=args.day
    } else {
        1..=puzzles.len()
    };

    let parts_to_run = if args.part != 0 {
        args.part..=args.part
    } else {
        1..=2
    };

    for day in days_to_run {
        let contents = utilities::read_input(day);
        for part in parts_to_run.clone() {
            let func = puzzles[day - 1][part - 1];
            match func(&contents) {
                Ok(result) => println!("{}", result),
                Err(err) => eprintln!("ERROR: {}", err),
            }
        }
    }

    Ok(())
}
