use crate::utilities;
use std::str::FromStr;

pub fn part_1() -> u32 {
    let bag = Set {
        red: 12,
        green: 13,
        blue: 14,
    };
    
    let contents = utilities::read_input(2);
    let games: Vec<Game> = contents.lines().map(|line| line.parse::<Game>().unwrap()).collect();
    let valid_games = games.iter().filter(|game| is_valid(game, &bag));
    valid_games.fold(0, |acc, game| acc + game.id)
}

pub fn part_2() -> u32 {
    let _contents = utilities::read_input(2);
    0
}

struct Game {
    id: u32,
    sets: Vec<Set>,
}

#[derive(Debug)]
struct Set {
    red: u32,
    blue: u32,
    green: u32,
}


#[derive(Debug)]
struct ParseGameError {}

impl FromStr for Game {
    type Err = ParseGameError;
    fn from_str(s: &str) -> Result<Game, Self::Err> {
        // Game 1: 4 blue, 16 green, 2 red; 5 red, 11 blue, 16 green; 9 green, 11 blue; 10 blue, 6 green, 4 red
        let mut segments = s.split(':');
        if let (Some(game), Some(rest)) = (segments.next(), segments.next()) {
            if let (Ok(id), Ok(sets)) = (game[5..].parse::<u32>(), rest.split(';').map(|set| set.parse::<Set>()).into_iter().collect::<Result<Vec<_>,_>>()) {
                return Ok(Game {
                    id,
                    sets,
                })
            }
        }

        Err(ParseGameError {})
    }
}

#[derive(Debug)]
struct ParseSetError {}

impl FromStr for Set {
    type Err = ParseSetError;
    fn from_str(s: &str) -> Result<Set, Self::Err> {
        // 4 blue, 16 green, 2 red

        let mut result = Set {
            red: 0,
            green: 0,
            blue: 0,
        };

        let mut colors = s.split(',');
        while let Some(color) = colors.next() {
            let mut num_and_color = color.split_whitespace();
            let num = num_and_color.next().unwrap().parse::<u32>().unwrap();
            let c = num_and_color.next().unwrap();

            match c {
                "red" => result.red = num,
                "green" => result.green = num,
                "blue" => result.blue = num,
                _ => panic!()
            }
        }
        Ok(result)
    }
}

fn is_valid(game: &Game, bag: &Set) -> bool {
    game.sets.iter().all(|set| set.red <= bag.red && set.green <= bag.green && set.blue <= bag.blue)
}
