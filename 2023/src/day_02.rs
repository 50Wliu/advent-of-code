use std::{collections::HashMap, str::FromStr};

pub fn part_1(contents: &str) -> Result<u64, String> {
    let bag = Set {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games = contents
        .lines()
        .map(|line| line.parse::<Game>())
        .collect::<Result<Vec<_>, _>>()?;
    let valid_games = games.iter().filter(|game| {
        game.sets
            .iter()
            .all(|set| set.red <= bag.red && set.green <= bag.green && set.blue <= bag.blue)
    });
    Ok(valid_games.fold(0, |acc, game| acc + game.id))
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    let games = contents
        .lines()
        .map(|line| line.parse::<Game>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(games
        .iter()
        .map(|game| {
            let min_bag = game.sets.iter().fold(
                Set {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                |mut acc, set| {
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
                },
            );

            min_bag.red * min_bag.green * min_bag.blue
        })
        .sum())
}

struct Game {
    id: u64,
    sets: Vec<Set>,
}

#[derive(Debug)]
struct Set {
    red: u64,
    blue: u64,
    green: u64,
}

impl FromStr for Game {
    type Err = String;
    fn from_str(s: &str) -> Result<Game, Self::Err> {
        // Game 1: 4 blue, 16 green, 2 red; 5 red, 11 blue, 16 green; 9 green, 11 blue; 10 blue, 6 green, 4 red
        let mut segments = s.split(':');
        let game = segments.next().ok_or("Missing game".to_string())?;
        let sets = segments.next().ok_or("Missing sets".to_string())?;
        let id = game
            .strip_prefix("Game ")
            .ok_or("Missing `Game` prefix".to_string())?
            .trim()
            .parse::<u64>()
            .map_err(|err| err.to_string())?;
        let sets = sets
            .split(';')
            .map(|set| set.parse::<Set>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Game { id, sets })
    }
}

impl FromStr for Set {
    type Err = String;
    fn from_str(s: &str) -> Result<Set, Self::Err> {
        // 4 blue, 16 green, 2 red

        let colors = s.split(',').try_fold(HashMap::new(), |mut acc, segment| {
            let mut num_and_color = segment.split_whitespace();
            let num = num_and_color
                .next()
                .ok_or("Missing number".to_string())?
                .parse::<u64>()
                .map_err(|err| err.to_string())?;
            let color = num_and_color.next().ok_or("Missing color".to_string())?;
            match acc.insert(color, num) {
                Some(_) => Err(format!("Duplicate color {}", color).to_string()),
                None => Ok(acc),
            }
        })?;

        Ok(Set {
            red: *colors.get("red").unwrap_or(&0),
            green: *colors.get("green").unwrap_or(&0),
            blue: *colors.get("blue").unwrap_or(&0),
        })
    }
}
