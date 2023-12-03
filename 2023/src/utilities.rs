use std::fs;

pub fn read_input(day: u8) -> String {
    let path = format!("inputs/day_{:02}.txt", day);
    fs::read_to_string(path).expect("Something went wrong reading the file")
}
