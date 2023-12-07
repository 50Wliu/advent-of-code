use std::str::FromStr;

pub fn part_1(contents: &String) -> u32 {
    contents
        .lines()
        .map(|line| line.parse::<Card>().unwrap())
        .fold(0, |acc, card| {
            acc + card.winning_numbers
                .iter()
                .filter(|winning_number| card.drawn_numbers.contains(winning_number))
                .fold(0, |acc, _| {
                    if acc == 0 {
                        1
                    } else {
                        acc * 2
                    }
                })
        })
}

pub fn part_2(contents: &String) -> u32 {
    let lines = contents.lines();
    let cards = lines.map(|line| line.parse::<Card>().unwrap()).collect::<Vec<_>>();

    let mut card_copies = vec![0; cards.len()];

    cards.iter().fold(0, |acc, card| {
        let copies = card.winning_numbers
            .iter()
            .filter(|winning_number| card.drawn_numbers.contains(winning_number))
            .count();
        for copy in 1..=copies {
            card_copies[card.index - 1 + copy] += 1 + card_copies[card.index - 1];
        }

        acc + 1 + card_copies[card.index - 1]
    })
}

#[derive(Debug)]
struct Card {
    index: usize,
    winning_numbers: Vec<u32>,
    drawn_numbers: Vec<u32>,
}

#[derive(Debug)]
struct ParseCardError {}

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segments = s.split(": ");
        if let (Some(raw_index), Some(raw_card_segments)) = (segments.next(), segments.next()) {
            if let Some(index_with_whitespace) = raw_index.strip_prefix("Card ") {
                if let Ok(index) = index_with_whitespace.trim().parse::<usize>() {
                    let mut cards = raw_card_segments.split(" | ").map(|segment| {
                        segment.split_whitespace().map(|x| x.parse::<u32>()).collect::<Result<Vec<_>, _>>()
                    });
                    if let (Some(Ok(winning_numbers)), Some(Ok(drawn_numbers))) = (cards.next(), cards.next()) {
                        return Ok(Card { index, winning_numbers, drawn_numbers });
                    }
                }
            }
        }
        Err(ParseCardError {})
    }
}
