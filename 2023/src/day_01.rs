use crate::utilities;
use regex::Regex;

pub fn part_1() -> u32 {
    let contents = utilities::read_input(1);
    contents.lines().fold(0, |acc, line| {
        let matches = line.matches(char::is_numeric).collect::<Vec<&str>>();
        acc +
            matches.first().unwrap().parse::<u32>().unwrap() * 10 +
            matches.last().unwrap().parse::<u32>().unwrap()
    })
}

pub fn part_2() -> u32 {
    let contents = utilities::read_input(1);

    let re = Regex::new(r"").unwrap();

    contents.lines().fold(0, |acc, line| {
        let first = line.chars().nth(line.find(char::is_numeric).unwrap()).unwrap();
        let last = line.chars().nth(line.rfind("one").unwrap()).unwrap();
        acc + 10 * first.to_digit(10).unwrap() + last.to_digit(10).unwrap()
    })
}
