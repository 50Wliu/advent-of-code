use std::{cell::Cell, cmp::Ordering, fmt::Display, str::FromStr};

// I hate this.
std::thread_local! {
    static J_IS_JOKER: Cell<bool> = const { Cell::new(false) };
}

pub fn part_1(contents: &str) -> Result<u64, String> {
    J_IS_JOKER.set(false);

    calculate_winnings(contents.lines())
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    J_IS_JOKER.set(true);

    calculate_winnings(contents.lines())
}

fn calculate_winnings<'a>(lines: impl Iterator<Item = &'a str>) -> Result<u64, String> {
    let mut hands = lines
        .map(|line| {
            let mut line = line.split_whitespace();

            let hand = line
                .next()
                .ok_or("Missing hand")?
                .parse::<Hand>()
                .map_err(|err| err.to_string())?;
            let bid = line
                .next()
                .ok_or("Missing bid")?
                .parse::<u64>()
                .map_err(|err| err.to_string())?;

            Ok::<_, String>((hand, bid))
        })
        .collect::<Result<Vec<_>, _>>()?;

    hands.sort_unstable();

    Ok(hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, (_, bid))| acc + bid * (i as u64 + 1)))
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Display for HandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandType::HighCard => write!(f, "High Card"),
            HandType::OnePair => write!(f, "One Pair"),
            HandType::TwoPair => write!(f, "Two Pair"),
            HandType::ThreeOfAKind => write!(f, "Three of a Kind"),
            HandType::FullHouse => write!(f, "Full House"),
            HandType::FourOfAKind => write!(f, "Four of a Kind"),
            HandType::FiveOfAKind => write!(f, "Five of a Kind"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    r#type: HandType,
    cards: [Card; 5],
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.r#type,
            self.cards
                .into_iter()
                .fold(String::new(), |acc, card| acc + &card.to_string())
        )
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.r#type.cmp(&other.r#type) {
            Ordering::Equal => {
                for (a, b) in self.cards.into_iter().zip(other.cards) {
                    match a.cmp(&b) {
                        Ordering::Equal => continue,
                        ordering => return ordering,
                    }
                }
                Ordering::Equal
            }
            ordering => ordering,
        }
    }
}

impl FromStr for Hand {
    type Err = String;
    fn from_str(s: &str) -> Result<Hand, Self::Err> {
        let cards: [Card; 5] = s
            .chars()
            .map(|card| Card { value: card })
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| "Cards did not have 5 elements")?;

        let jokers = if J_IS_JOKER.get() {
            cards.into_iter().filter(|card| card.value == 'J').count()
        } else {
            0
        };

        let mut sorted_cards = cards;
        sorted_cards.sort_unstable();

        match sorted_cards {
            [a, b, c, d, e] if a == b && b == c && c == d && d == e => Ok(Hand {
                r#type: HandType::FiveOfAKind,
                cards,
            }),
            [a, b, c, d, _] if a == b && b == c && c == d => Ok(Hand {
                r#type: HandType::FourOfAKind,
                cards,
            }),
            [_, b, c, d, e] if b == c && c == d && d == e => Ok(Hand {
                r#type: HandType::FourOfAKind,
                cards,
            }),
            [a, b, c, d, e] if a == b && c == d && d == e => Ok(Hand {
                r#type: if jokers == 2 || (jokers == 1 && a.value == 'J') {
                    HandType::FourOfAKind // JJQAA, J5TTT
                } else {
                    HandType::FullHouse
                },
                cards,
            }),
            [a, b, c, d, e] if a == b && b == c && d == e => Ok(Hand {
                r#type: if jokers == 2 {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                },
                cards,
            }),
            [a, b, c, _, _] if a == b && b == c => Ok(Hand {
                r#type: HandType::ThreeOfAKind, // d & e can't be jokers, e.g. J4456
                cards,
            }),
            [_, b, c, d, _] if b == c && c == d => Ok(Hand {
                r#type: if jokers == 1 {
                    HandType::FourOfAKind
                } else {
                    HandType::ThreeOfAKind
                },
                cards,
            }),
            [_, _, c, d, e] if c == d && d == e => Ok(Hand {
                r#type: if jokers == 1 {
                    HandType::FourOfAKind // JQKKK
                } else {
                    HandType::ThreeOfAKind
                },
                cards,
            }),
            [a, b, c, d, _] if a == b && c == d => Ok(Hand {
                r#type: if jokers == 1 {
                    HandType::ThreeOfAKind //J2QQK
                } else {
                    HandType::TwoPair
                },
                cards,
            }),
            [a, b, _, d, e] if a == b && d == e => Ok(Hand {
                r#type: if jokers == 1 {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                },
                cards,
            }),
            [_, b, c, d, e] if b == c && d == e => Ok(Hand {
                r#type: if jokers == 1 {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                },
                cards,
            }),
            [a, b, c, d, e] if a != b && b != c && c != d && d != e => Ok(Hand {
                r#type: HandType::HighCard,
                cards,
            }),
            _ => Ok(Hand {
                r#type: HandType::OnePair,
                cards,
            }),
        }
    }
}

#[derive(Debug, Eq, Clone, Copy)]
struct Card {
    value: char,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        if J_IS_JOKER.get() {
            return self.value == other.value || self.value == 'J' || other.value == 'J';
        }
        self.value == other.value
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.value {
            'A' => match other.value {
                'A' => Ordering::Equal,
                _ => Ordering::Greater,
            },
            'K' => match other.value {
                'A' => Ordering::Less,
                'K' => Ordering::Equal,
                _ => Ordering::Greater,
            },
            'Q' => match other.value {
                'A' | 'K' => Ordering::Less,
                'Q' => Ordering::Equal,
                _ => Ordering::Greater,
            },
            'J' if J_IS_JOKER.get() => match other.value {
                'J' => Ordering::Equal,
                _ => Ordering::Less,
            },
            'J' => match other.value {
                'A' | 'K' | 'Q' => Ordering::Less,
                'J' => Ordering::Equal,
                _ => Ordering::Greater,
            },
            'T' => match other.value {
                'A' | 'K' | 'Q' => Ordering::Less,
                'J' => {
                    if J_IS_JOKER.get() {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                }
                'T' => Ordering::Equal,
                _ => Ordering::Greater,
            },
            _ => match other.value {
                'A' | 'K' | 'Q' | 'T' => Ordering::Less,
                'J' => {
                    if J_IS_JOKER.get() {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                }
                _ => self.value.cmp(&other.value),
            },
        }
    }
}
