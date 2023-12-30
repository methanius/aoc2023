fn main() {
    let text =
        std::fs::read_to_string("data/04a.txt").expect("Couldn't read file at hard-coded path!");
    part_1(&text);
    println!("{:?}", part_2(&text));
}

fn part_1(block: &str) {
    println!(
        "{:?}",
        block.lines().map(parse_card_to_points).sum::<usize>()
    );
}

fn parse_line_to_card_number_et_num_matches(card: &str) -> (usize, usize) {
    let card_data: Vec<&str> = card.split(|c| c == ':' || c == '|').collect();
    let index = card_data[0]
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let all_numbers: Vec<Vec<usize>> = card_data
        .iter()
        .skip(1)
        .map(|s| s.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect();
    (
        index,
        all_numbers[1]
            .iter()
            .filter_map(|n| {
                if all_numbers[0].contains(n) {
                    Some(1)
                } else {
                    None
                }
            })
            .sum(),
    )
}

fn parse_card_to_points(card: &str) -> usize {
    let numbers = card
        .split(|c| c == ':' || c == '|')
        .skip(1)
        .map(|s| s.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect::<Vec<Vec<usize>>>();
    match numbers[1]
        .iter()
        .filter(|n| numbers[0].contains(n))
        .collect::<Vec<&usize>>()
    {
        v if v.is_empty() => 0,
        v if v.len() == 1 => 1,
        v if v.len() > 1 => v.iter().skip(1).fold(1, |acc, _| acc * 2),
        _ => panic!("This should never happen"),
    }
}

struct ParserData<'a> {
    cards_to_match: Vec<(usize, usize)>,
    list_of_cards: &'a Vec<(usize, usize)>,
    accumulated_cards: usize,
}

fn recusively_collect_cards(data: ParserData) -> ParserData {
    if data.cards_to_match.is_empty() {
        data
    } else {
        recusively_collect_cards(ParserData {
            accumulated_cards: data.accumulated_cards + data.cards_to_match.len(),
            cards_to_match: data
                .cards_to_match
                .iter()
                .flat_map(|(ind, points)| {
                    data.list_of_cards
                        .iter()
                        .take(ind + points)
                        .skip(*ind)
                        .copied()
                        .collect::<Vec<_>>()
                })
                .collect(),
            ..data
        })
    }
}

fn part_2(block: &str) -> usize {
    let indeces_et_points: Vec<(usize, usize)> = block
        .lines()
        .map(parse_line_to_card_number_et_num_matches)
        .collect();
    recusively_collect_cards(ParserData {
        cards_to_match: indeces_et_points.clone(),
        list_of_cards: &indeces_et_points,
        accumulated_cards: 0,
    })
    .accumulated_cards
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn day4_card_to_points() {
        assert_eq!(parse_card_to_points(INPUT.lines().next().unwrap()), 8);
    }

    #[test]
    fn day4_input_total_card_points() {
        assert_eq!(INPUT.lines().map(parse_card_to_points).sum::<usize>(), 13);
    }

    #[test]
    fn day4_part2_test_input() {
        assert_eq!(part_2(INPUT), 30);
    }
}
