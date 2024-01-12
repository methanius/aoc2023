use std::collections::HashMap;

fn main() {
    let text =
        std::fs::read_to_string("data/07a.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part one result: {:?}", part1(&text));
    println!("Part two result: {:?}", part2(&text));
}

fn part2(block: &str) -> usize {
    Game::CamelPokerWithJokers.winnings_in_block(block)
}

fn part1(block: &str) -> usize {
    Game::CamelPokerWithoutJokers.winnings_in_block(block)
}

enum Game {
    CamelPokerWithoutJokers,
    CamelPokerWithJokers,
}

impl Game {
    fn get_card_ranks(&self, cards_in_hand: &[Card]) -> Vec<usize> {
        match self {
            Self::CamelPokerWithoutJokers => {
                let matcher = [
                    Card::Two,
                    Card::Three,
                    Card::Four,
                    Card::Five,
                    Card::Six,
                    Card::Seven,
                    Card::Eight,
                    Card::Nine,
                    Card::T,
                    Card::J,
                    Card::Q,
                    Card::K,
                    Card::A,
                ];
                cards_in_hand
                    .iter()
                    .map(|&c| matcher.iter().position(|&m| m == c).unwrap())
                    .collect()
            }
            Self::CamelPokerWithJokers => {
                let matcher = [
                    Card::J,
                    Card::Two,
                    Card::Three,
                    Card::Four,
                    Card::Five,
                    Card::Six,
                    Card::Seven,
                    Card::Eight,
                    Card::Nine,
                    Card::T,
                    Card::Q,
                    Card::K,
                    Card::A,
                ];
                cards_in_hand
                    .iter()
                    .map(|&c| matcher.iter().position(|&m| m == c).unwrap())
                    .collect()
            }
        }
    }

    fn score_cards(&self, cards_in_hand: &[Card]) -> (HandType, Vec<usize>) {
        (
            self.get_handtype(&Self::count_cards(cards_in_hand)),
            self.get_card_ranks(cards_in_hand),
        )
    }

    fn count_cards(cards_in_hand: &[Card]) -> HashMap<Card, usize> {
        cards_in_hand.iter().fold(HashMap::new(), |mut map, card| {
            *map.entry(*card).or_insert(0) += 1;
            map
        })
    }

    fn get_handtype(&self, card_counts: &HashMap<Card, usize>) -> HandType {
        match self {
            Self::CamelPokerWithoutJokers => {
                let mut counts: Vec<&usize> = card_counts.values().collect();
                counts.sort_unstable();
                match counts.pop().unwrap() {
                    5 => HandType::FiveOfAKind,
                    4 => HandType::FourOfAKind,
                    3 => match counts.pop().unwrap() {
                        2 => HandType::FullHouse,
                        1 => HandType::ThreeOfAKind,
                        _ => panic!("Impossible outcome"),
                    },
                    2 => match counts.pop().unwrap() {
                        2 => HandType::TwoPair,
                        1 => HandType::OnePair,
                        _ => panic!("Impossible outcome"),
                    },
                    1 => HandType::HighCard,
                    _ => panic!("Impossible outcome"),
                }
            }
            Self::CamelPokerWithJokers => {
                let mut mutable_card_count = card_counts.clone();
                let jokers = mutable_card_count.remove(&Card::J).unwrap_or(0);
                let mut counts: Vec<&usize> = mutable_card_count.values().collect();
                counts.sort_unstable();
                match (jokers, counts.pop().unwrap_or(&0)) {
                    (5, 0) | (4, 1) | (3, 2) | (2, 3) | (1, 4) | (0, 5) => HandType::FiveOfAKind,
                    (3, 1) | (2, 2) | (1, 3) | (0, 4) => HandType::FourOfAKind,
                    (2, 1) => HandType::ThreeOfAKind,
                    (1, 1) => HandType::OnePair,
                    (1, 2) | (0, 3) => match counts.pop().unwrap() {
                        2 => HandType::FullHouse,
                        1 => HandType::ThreeOfAKind,
                        _ => panic!("Impossible outcome"),
                    },
                    (0, 2) => match counts.pop().unwrap() {
                        2 => HandType::TwoPair,
                        1 => HandType::OnePair,
                        _ => panic!("Impossible scenario"),
                    },
                    (0, 1) => HandType::HighCard,
                    _ => panic!("Impossible scenario"),
                }
            }
        }
    }
    fn winnings_in_block(&self, block: &str) -> usize {
        let mut lines: Vec<(Vec<Card>, usize)> = block
            .lines()
            .map(|line| line.split(' ').collect())
            .map(|elms: Vec<&str>| {
                (
                    elms[0]
                        .chars()
                        .map(|c| Card::try_from(c).unwrap())
                        .collect(),
                    elms[1].parse().unwrap(),
                )
            })
            .collect();
        lines.sort_unstable_by_key(|tup| self.score_cards(&tup.0));
        lines
            .iter()
            .enumerate()
            .fold(0, |acc, (n, tup)| acc + (n + 1) * tup.1)
    }
}

#[derive(PartialOrd, PartialEq, Eq, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    const fn try_from(c: char) -> Option<Self> {
        match c {
            '2' => Some(Self::Two),
            '3' => Some(Self::Three),
            '4' => Some(Self::Four),
            '5' => Some(Self::Five),
            '6' => Some(Self::Six),
            '7' => Some(Self::Seven),
            '8' => Some(Self::Eight),
            '9' => Some(Self::Nine),
            'T' => Some(Self::T),
            'J' => Some(Self::J),
            'Q' => Some(Self::Q),
            'K' => Some(Self::K),
            'A' => Some(Self::A),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn day7_part1_test() {
        assert_eq!(part1(INPUT), 6440);
    }

    #[test]
    fn day7_part2_test() {
        assert_eq!(part2(INPUT), 5905);
    }
}
