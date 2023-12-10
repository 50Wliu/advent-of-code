use std::str::FromStr;

pub fn part_1(contents: &str) -> Result<u64, String> {
    let cards = contents
        .lines()
        .map(|line| line.parse::<Card>())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(cards.iter().fold(0, |acc, card| {
        acc + card
            .winning_numbers
            .iter()
            .filter(|winning_number| card.drawn_numbers.contains(winning_number))
            .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
    }))
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    let lines = contents.lines();
    let cards = lines
        .map(|line| line.parse::<Card>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut card_copies = vec![0; cards.len()];

    Ok(cards.iter().fold(0, |acc, card| {
        let copies = card
            .winning_numbers
            .iter()
            .filter(|winning_number| card.drawn_numbers.contains(winning_number))
            .count();
        for copy in 1..=copies {
            card_copies[card.index - 1 + copy] += 1 + card_copies[card.index - 1];
        }

        acc + 1 + card_copies[card.index - 1]
    }))
}

#[derive(Debug)]
struct Card {
    index: usize,
    winning_numbers: Vec<u64>,
    drawn_numbers: Vec<u64>,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segments = s.split(": ");
        let index = segments
            .next()
            .ok_or("Missing card index")?
            .strip_prefix("Card ")
            .ok_or("Missing `Card` prefix")?
            .trim()
            .parse::<usize>()
            .map_err(|err| err.to_string())?;
        let mut numbers = segments.next().ok_or("Missing numbers")?.split(" | ");
        let winning_numbers = numbers
            .next()
            .ok_or("Missing winning numbers")?
            .split_whitespace()
            .map(|card| card.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| err.to_string())?;
        let drawn_numbers = numbers
            .next()
            .ok_or("Missing drawn numbers")?
            .split_whitespace()
            .map(|card| card.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| err.to_string())?;

        Ok(Card {
            index,
            winning_numbers,
            drawn_numbers,
        })
    }
}
