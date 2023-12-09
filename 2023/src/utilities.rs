use std::fs;

pub fn read_input(day: usize) -> Result<String, String> {
    let path = format!("inputs/day_{:02}.txt", day);
    fs::read_to_string(path).map_err(|err| err.to_string())
}
