use std::iter::zip;

fn main() {
    let text =
        std::fs::read_to_string("data/06a.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part one result: {:?}", part1(&text));
    println!("Part two result: {:?}", part2(&text));
}

fn part1(block: &str) -> i64 {
    let parsed_ints: Vec<_> = block
        .lines()
        .map(|s| {
            s.split(':')
                .nth(1)
                .unwrap()
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    zip(&parsed_ints[0], &parsed_ints[1])
        .map(|(time, dist)| Game {
            time: *time,
            dist: *dist,
        })
        .map(|g| g.winning_int_time_range())
        .map(|(first, last)| last - first + 1)
        .reduce(|acc, e| acc * e)
        .unwrap()
}

fn part2(block: &str) -> i64 {
    let parsed_ints: Vec<_> = block
        .lines()
        .map(|s| {
            s.split(':')
                .nth(1)
                .unwrap()
                .split(' ')
                .filter(|s| !s.is_empty())
                .collect::<String>()
                .parse::<i64>()
                .unwrap()
        })
        .collect();
    let winning_tuple = Game {
        time: parsed_ints[0],
        dist: parsed_ints[1],
    }
    .winning_int_time_range();
    winning_tuple.1 - winning_tuple.0 + 1
}

#[derive(Debug)]
struct Game {
    time: i64,
    dist: i64,
}

impl Game {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::float_cmp)]
    #[allow(clippy::cast_precision_loss)]
    fn winning_int_time_range(&self) -> (i64, i64) {
        let sqrtdd2: f64 =
            (4.0f64.mul_add(-(self.dist as f64), (self.time * self.time) as f64)).sqrt() / 2.0;
        let adjust = i64::from(((sqrtdd2 * 10.0).round() / 10.0).trunc() == sqrtdd2);
        let low = (self.time as f64 / 2.0 - sqrtdd2).ceil() as i64 + adjust;
        let high = (self.time as f64 / 2.0 + sqrtdd2).floor() as i64 - adjust;
        (low, high)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn day6_part1() {
        assert_eq!(part1(INPUT), 288);
    }
    #[test]
    fn day6_part2() {
        assert_eq!(part2(INPUT), 71503);
    }
}
