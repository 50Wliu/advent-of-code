use std::str::FromStr;

pub fn part_1() -> u32 {
    let contents = super::utilities::read_input(7);
    let mut chars = contents.trim().chars().skip(1); // Skip past initial $
    while let Some(command) = parse_next_command(&mut chars) {
        println!("{:?}", command);
    }
    0
}

pub fn part_2() -> u32 {
    0
}

fn parse_next_command<I>(contents: &mut I) -> Option<Command>
where
    I: Iterator<Item = char>,
{
    let contents = contents.skip_while(|c| c.is_whitespace());
    let command = contents.take_while(|c| *c != '$').collect::<String>();
    if command.len() > 0 {
        return Some(command.parse::<Command>().expect("Command should be valid"));
    }
    None
}

enum DirEntry {
    Directory(String, Vec<DirEntry>, u32),
    File(String, u32),
}

#[derive(Debug)]
struct ParseDirEntryError {}

impl FromStr for DirEntry {
    type Err = ParseDirEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segments = s.split_whitespace();
        if let (Some(size), Some(name), None) = (segments.next(), segments.next(), segments.next())
        {
            if size == "dir" {
                return Ok(DirEntry::Directory(name.to_string(), Vec::new(), 0));
            }

            if let Ok(size) = size.parse::<u32>() {
                return Ok(DirEntry::File(name.to_string(), size));
            }
        }

        Err(ParseDirEntryError {})
    }
}

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls(Vec<String>),
}

#[derive(Debug)]
struct ParseCommandError {}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        if let Some(input) = lines.next() {
            let input = input.trim();
            if input == "ls" {
                return Ok(Command::Ls(lines.map(str::to_string).collect()));
            }

            let mut segments = input.split_whitespace();
            if let (Some("cd"), Some(directory), None) =
                (segments.next(), segments.next(), segments.next())
            {
                return Ok(Command::Cd(directory.to_string()));
            }
        }

        Err(ParseCommandError {})
    }
}
