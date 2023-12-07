use std::str::FromStr;

pub fn part_1(contents: &String) -> Result<u32, String> {
    let bag = Set {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games = contents.lines().map(|line| line.parse::<Game>()).collect::<Result<Vec<_>, _>>()?;
    let valid_games = games.iter().filter(|game|
        game.sets.iter().all(|set| set.red <= bag.red && set.green <= bag.green && set.blue <= bag.blue));
    Ok(valid_games.fold(0, |acc, game| acc + game.id))
}

pub fn part_2(contents: &String) -> Result<u32, String> {
    let games = contents.lines().map(|line| line.parse::<Game>()).collect::<Result<Vec<_>, _>>()?;
    Ok(games.iter().map(|game| {
        let min_bag = game.sets.iter().fold(Set { red: 0, green: 0, blue: 0 }, |mut acc, set| {
            if set.red > acc.red {
                acc.red = set.red;
            }

            if set.green > acc.green {
                acc.green = set.green;
            }

            if set.blue > acc.blue {
                acc.blue = set.blue;
            }

            acc
        });

        min_bag.red * min_bag.green * min_bag.blue
    }).sum())
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

impl FromStr for Game {
    type Err = String;
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

        Err("Invalid game".to_string())
    }
}

impl FromStr for Set {
    type Err = String;
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
            if let (Some(raw_num), Some(color)) = (num_and_color.next(), num_and_color.next()) {
                let num = raw_num.parse::<u32>().map_err(|err| err.to_string())?;
                match color {
                    "red" => result.red = num,
                    "green" => result.green = num,
                    "blue" => result.blue = num,
                    _ => panic!()
                }
            }
        }
        Ok(result)
    }
}
