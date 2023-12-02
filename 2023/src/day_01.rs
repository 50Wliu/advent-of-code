use crate::utilities;

pub fn part_1() -> u32 {
    let contents = utilities::read_input(1);
    contents.lines().fold(0, |acc, line| {
        let first = line.chars().nth(line.find(char::is_numeric).unwrap()).unwrap();
        let last = line.chars().nth(line.rfind(char::is_numeric).unwrap()).unwrap();
        acc + 10 * first.to_digit(10).unwrap() + last.to_digit(10).unwrap()
    })
}

pub fn part_2() -> u32 {
    let contents = utilities::read_input(1);
    0
}
