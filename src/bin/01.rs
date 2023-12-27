fn match_number_start_part_1(line: &str) -> Option<u32> {
    match line.chars().next().unwrap() {
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        _ => None,
    }
}

fn match_number_start_part_2(line: &str) -> Option<u32> {
    match line {
        _ if line.starts_with('1') => Some(1),
        _ if line.starts_with('2') => Some(2),
        _ if line.starts_with('3') => Some(3),
        _ if line.starts_with('4') => Some(4),
        _ if line.starts_with('5') => Some(5),
        _ if line.starts_with('6') => Some(6),
        _ if line.starts_with('7') => Some(7),
        _ if line.starts_with('8') => Some(8),
        _ if line.starts_with('9') => Some(9),
        _ if line.starts_with("one") => Some(1),
        _ if line.starts_with("two") => Some(2),
        _ if line.starts_with("three") => Some(3),
        _ if line.starts_with("four") => Some(4),
        _ if line.starts_with("five") => Some(5),
        _ if line.starts_with("six") => Some(6),
        _ if line.starts_with("seven") => Some(7),
        _ if line.starts_with("eight") => Some(8),
        _ if line.starts_with("nine") => Some(9),
        _ => None,
    }
}

fn parse_string_with_matcher(line: &str, matcher: &dyn Fn(&str) -> Option<u32>) -> u32 {
    let (_, first, last, _) = iteratively_parse_string_line_for_numbers(line, None, None, &matcher);
    first.expect("All strings should contain numbers") * 10
        + last.expect("All strings should contain numbers")
}

type RecurserState<'a> = (
    &'a str,
    Option<u32>,
    Option<u32>,
    &'a dyn Fn(&str) -> Option<u32>,
);

fn parse_string_part_1(line: &str) -> u32 {
    parse_string_with_matcher(line, &match_number_start_part_1)
}

fn parse_string_part_2(line: &str) -> u32 {
    parse_string_with_matcher(line, &match_number_start_part_2)
}

fn iteratively_parse_string_line_for_numbers<'a>(
    line: &'a str,
    first: Option<u32>,
    last: Option<u32>,
    matcher: &'a dyn Fn(&str) -> Option<u32>,
) -> RecurserState<'a> {
    match line.len() {
        0 => ("", first, last, matcher),
        _ => matcher(line).map_or_else(
            || iteratively_parse_string_line_for_numbers(&line[1..], first, last, matcher),
            |num| {
                let first = Some(first.unwrap_or(num));
                let last = Some(num);
                iteratively_parse_string_line_for_numbers(&line[1..], first, last, matcher)
            },
        ),
    }
}

fn part_1() {
    let text =
        std::fs::read_to_string("data/01a.txt").expect("Couldn't read file at hard-coded path!");
    let sum: u32 = text.lines().map(parse_string_part_1).sum();
    println!("Part 1 -> Sum: {sum}");
    let sum_2: u32 = text.lines().map(parse_string_part_2).sum();
    println!("Part 2 -> Sum: {sum_2}");
}

fn main() {
    part_1();
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn day1_part1_numbers_vector() {
        let line = INPUT.lines().next().map(parse_string_part_1);
        assert_eq!(line, Some(12));
        let numbers_vector = INPUT.lines().map(parse_string_part_1).collect::<Vec<u32>>();
        assert_eq!(vec![12, 38, 15, 77], numbers_vector);
        let sum = numbers_vector.iter().sum::<u32>();
        assert_eq!(sum, 142);
    }

    const INPUT2: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn day1_part2_parse_single_line() {
        let line = INPUT2.lines().next().map(parse_string_part_2);
        assert_eq!(line, Some(29));
    }
    #[test]
    fn day1_part2_parse_all_lines_and_sum() {
        let numbers_vector = INPUT2
            .lines()
            .map(parse_string_part_2)
            .collect::<Vec<u32>>();
        assert_eq!(vec![29, 83, 13, 24, 42, 14, 76], numbers_vector);
        let sum = numbers_vector.iter().sum::<u32>();
        assert_eq!(sum, 281);
    }
}
