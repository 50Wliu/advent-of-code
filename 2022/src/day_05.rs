use std::str::FromStr;

pub fn part_1() -> String {
    let contents = super::utilities::read_input(5);
    let mut supplies = contents.lines().take_while(|line| {
        !line.is_empty()
    }).collect::<Vec<&str>>();
    let num_stacks = supplies.remove(supplies.len() - 1).split_whitespace().count();

    let mut stacks = vec![Vec::<char>::new(); num_stacks];
    for line in supplies.iter().rev() {
        let row = line.chars();
        for (i, krate) in row.enumerate().skip(1).step_by(4) {
            if !krate.is_whitespace() {
                stacks[(i - 1) / 4].push(krate);
            }
        }
    }

    let instructions = contents.lines().skip_while(|line| {
        !line.is_empty()
    }).skip(1);
    for line in instructions {
        let instruction = line.parse::<Instruction>().expect("Each line should contain a valid instruction");
        for _ in 0..instruction.num_crates {
            let krate = stacks[instruction.source].pop().expect("Stack should not be empty");
            stacks[instruction.destination].push(krate);
        }
    }

    stacks.iter().map(|stack| {
        stack.last().expect("Stack should not be empty")
    }).collect()
}

pub fn part_2() -> String {
    let contents = super::utilities::read_input(5);
    let mut supplies = contents.lines().take_while(|line| {
        !line.is_empty()
    }).collect::<Vec<&str>>();
    let num_stacks = supplies.remove(supplies.len() - 1).split_whitespace().count();

    let mut stacks = vec![Vec::<char>::new(); num_stacks];
    for line in supplies.iter().rev() {
        let row = line.chars();
        for (i, krate) in row.enumerate().skip(1).step_by(4) {
            if !krate.is_whitespace() {
                stacks[(i - 1) / 4].push(krate);
            }
        }
    }

    let instructions = contents.lines().skip_while(|line| {
        !line.is_empty()
    }).skip(1);
    for line in instructions {
        let instruction = line.parse::<Instruction>().expect("Each line should contain a valid instruction");
        let stack_length = stacks[instruction.source].len();
        let mut crates = stacks[instruction.source].drain((stack_length - instruction.num_crates)..).collect();
        stacks[instruction.destination].append(&mut crates);
    }

    stacks.iter().map(|stack| {
        stack.last().expect("Stack should not be empty")
    }).collect()
}

struct Instruction {
    source: usize,
    destination: usize,
    num_crates: usize
}

#[derive(Debug)]
struct ParseInstructionError {}

impl FromStr for Instruction {
    type Err = ParseInstructionError;
    fn from_str(str: &str) -> Result<Instruction, Self::Err> {
        let mut segments = str.split_whitespace().skip(1).step_by(2);
        if let (Some(num_crates), Some(source), Some(destination)) = (segments.next(), segments.next(), segments.next()) {
            if let (Ok(num_crates), Ok(source), Ok(destination)) = (num_crates.parse::<usize>(), source.parse::<usize>(), destination.parse::<usize>()) {
                return Ok(Instruction {
                    source: source - 1,
                    destination: destination - 1,
                    num_crates
                })
            }
        }

        Err(ParseInstructionError {})
    }
}
