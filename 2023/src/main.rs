pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;

pub mod utilities;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    // Day to run
    #[arg(short = 'd', long)]
    day: Option<usize>,

    // Part to run
    #[arg(short = 'p', long)]
    part: Option<usize>,
}

fn main() -> Result<(), String> {
    let args = Cli::parse();

    let puzzles = [
        [day_01::part_1, day_01::part_2],
        [day_02::part_1, day_02::part_2],
        [day_03::part_1, day_03::part_2],
        [day_04::part_1, day_04::part_2],
    ];

    let days_to_run = match args.day {
        Some(day) if day <= puzzles.len() => day..=day,
        Some(day) => return Err(format!("day {} not yet registered in main.rs", day)),
        None => 1..=puzzles.len(),
    };

    let parts_to_run = match args.part {
        Some(part) if part == 1 || part == 2 => part..=part,
        Some(part) => return Err(format!("part {} does not exist", part)),
        None => 1..=2,
    };

    for day in days_to_run {
        let contents = utilities::read_input(day)?;
        for part in parts_to_run.clone() {
            let func = puzzles[day - 1][part - 1];
            println!("{}", func(&contents)?);
        }
    }

    Ok(())
}
