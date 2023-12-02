use std::fs;

pub fn read_input(day: u8) -> String {
    let path = format!("day_{:02}_input.txt", day);
    fs::read_to_string(path).expect("Something went wrong reading the file")
}
