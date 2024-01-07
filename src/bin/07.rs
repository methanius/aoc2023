use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let text =
        std::fs::read_to_string("data/07a.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part one result: {:?}", part1(&text));
    // println!("Part two result: {:?}", part2(&text));
}

fn part1(block: &str) -> usize {
    let mut games: Vec<Game> = block.lines().map(Game::from_line).collect();
    games.sort();
    games
        .iter()
        .enumerate()
        .fold(0, |acc, (rank, game)| acc + (rank + 1) * game.bid)
}

#[derive(Debug, Clone)]
struct Game {
    hand: Hand,
    bid: usize,
}

#[derive(Debug, Clone, PartialOrd)]
struct Hand {
    handtype: HandType,
    cards: Vec<Card>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone, Copy)]
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

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Card {
    fn card_from_str(c: char) -> Self {
        match c {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => panic!("Unexpected char encountered"),
        }
    }
}

impl HandType {
    fn from_cards(cards: &[Card]) -> Self {
        let mut occurrences: Vec<usize> = cards
            .iter()
            .fold(HashMap::<Card, usize>::new(), |mut m, n| {
                *m.entry(*n).or_default() += 1;
                m
            })
            .into_values()
            .collect();
        occurrences.sort_unstable();
        match occurrences.pop().unwrap() {
            5 => Self::FiveOfAKind,
            4 => Self::FourOfAKind,
            3 => match occurrences.pop().unwrap() {
                2 => Self::FullHouse,
                1 => Self::ThreeOfAKind,
                _ => panic!("Impossible number from sorted list!"),
            },
            2 => match occurrences.pop().unwrap() {
                2 => Self::TwoPair,
                1 => Self::OnePair,
                _ => panic!("Impossible number from sorted list!"),
            },
            1 => Self::HighCard,
            _ => panic!("Impossible number from sorted list!"),
        }
    }
}

impl Hand {
    fn from_word(word: &str) -> Self {
        let cards: Vec<Card> = word.chars().map(Card::card_from_str).collect();
        Self {
            cards: cards.clone(),
            handtype: HandType::from_cards(&cards),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Game {
    fn from_line(line: &str) -> Self {
        let words: Vec<&str> = line.split(' ').collect();
        Self {
            hand: Hand::from_word(words[0]),
            bid: words[1].parse().unwrap(),
        }
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}

impl Eq for Game {}
impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
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
    fn day7_hand_orderings() {
        assert_eq!(Hand::from_word("AAKKK") > Hand::from_word("KKQQQ"), true);
        assert_eq!(Hand::from_word("KKQQQ") == Hand::from_word("KKQQQ"), true);
        assert_eq!(Hand::from_word("QKKQQ") < Hand::from_word("KKQQQ"), true);
        assert_eq!(Hand::from_word("AKQJT") < Hand::from_word("22333"), true);
        assert_eq!(Hand::from_word("KK677") > Hand::from_word("KTJJT"), true);
    }

    #[test]
    fn day7_game_ordering() {
        let games: Vec<Game> = INPUT.lines().map(Game::from_line).collect();
        let mut ordered_games = games.to_owned();
        ordered_games.sort();
        println!("{ordered_games:?}");
        assert_eq!(
            vec![
                games[0].clone(),
                games[3].clone(),
                games[2].clone(),
                games[1].clone(),
                games[4].clone()
            ] == ordered_games,
            true
        );
    }
}
