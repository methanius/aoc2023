fn main() {
    let text =
        std::fs::read_to_string("data/09a.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part one result: {:?}", part1(&text));
    println!("Part two result: {:?}", part2(&text));
}

fn part1(block: &str) -> i64 {
    read_block(block)
        .iter()
        .map(|l| recurse_next_number(l))
        .sum()
}

fn part2(block: &str) -> i64 {
    read_block(block)
        .iter()
        .map(|l| recurse_previous_number(l))
        .sum()
}

fn recurse_next_number(numbers: &[i64]) -> i64 {
    match numbers.iter().filter(|&n| n != &0).count() {
        0 => 0,
        _ => {
            numbers.last().unwrap()
                + recurse_next_number(
                    &numbers
                        .windows(2)
                        .map(|chk| chk[1] - chk[0])
                        .collect::<Vec<i64>>(),
                )
        }
    }
}

fn recurse_previous_number(numbers: &[i64]) -> i64 {
    match numbers.iter().filter(|&n| n != &0).count() {
        0 => 0,
        _ => {
            numbers.first().unwrap()
                - recurse_previous_number(
                    &numbers
                        .windows(2)
                        .map(|chk| chk[1] - chk[0])
                        .collect::<Vec<i64>>(),
                )
        }
    }
}

fn read_block(block: &str) -> Vec<Vec<i64>> {
    block
        .lines()
        .map(|s| {
            s.split(' ')
                .map(|c| c.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT1: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    // #[test]
    // fn day9_part1_test1() {
    //     assert_eq!(part1(INPUT1), 114);
    // }

    #[test]
    fn day9_part1_test_recurser() {
        assert_eq!(
            read_block(INPUT1)
                .iter()
                .map(|l| recurse_next_number(l))
                .collect::<Vec<i64>>(),
            vec![18, 28, 68]
        )
    }

    #[test]
    fn day9_part2_test_recurser() {
        assert_eq!(
            read_block(INPUT1)
                .iter()
                .map(|l| recurse_previous_number(l))
                .collect::<Vec<i64>>(),
            vec![-3, 0, 5]
        )
    }
}
