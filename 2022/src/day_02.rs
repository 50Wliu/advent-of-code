use std::cmp::Ordering;

pub fn part_1() -> u32 {
    let contents = super::utilities::read_input(2);
    contents.lines().map(|line| {
        line.split_whitespace().collect::<Vec<&str>>()
    }).fold(0, |accum, round| {
        let opponent_shape = match round[0] {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("Invalid opponent shape"),
        };
        let self_shape = match round[1] {
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => panic!("Invalid self shape"),
        };

        let result = self_shape.cmp(&opponent_shape);
        accum + self_shape as u32 + match result {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        }
    })
}

pub fn part_2() -> u32 {
    let contents = super::utilities::read_input(2);
    contents.lines().map(|line| {
        line.split_whitespace().collect::<Vec<&str>>()
    }).fold(0, |accum, round| {
        let opponent_shape = match round[0] {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("Invalid opponent shape"),
        };
        let outcome = match round[1] {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Invalid outcome"),
        };

        let self_shape = Shape::from_i32((opponent_shape as i32 + outcome as i32) % 3);

        let result = self_shape.cmp(&opponent_shape);
        accum + self_shape as u32 + match result {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        }
    })
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    fn from_i32(value: i32) -> Shape {
        match value {
            0 => Shape::Scissors, // Edge case since we're one-indexed
            1 => Shape::Rock,
            2 => Shape::Paper,
            3 => Shape::Scissors,
            _ => panic!("Invalid shape"),
        }
    }
}

impl Ord for Shape {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Shape::Rock => match other {
                Shape::Rock => Ordering::Equal,
                Shape::Paper => Ordering::Less,
                Shape::Scissors => Ordering::Greater,
            },
            Shape::Paper => match other {
                Shape::Rock => Ordering::Greater,
                Shape::Paper => Ordering::Equal,
                Shape::Scissors => Ordering::Less,
            },
            Shape::Scissors => match other {
                Shape::Rock => Ordering::Less,
                Shape::Paper => Ordering::Greater,
                Shape::Scissors => Ordering::Equal,
            },
        }
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

enum Outcome {
    Lose = -1,
    Draw = 0,
    Win = 1,
}
