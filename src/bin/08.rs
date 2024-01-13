use std::collections::HashMap;

fn main() {
    let text =
        std::fs::read_to_string("data/08a.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part one result: {:?}", part1(&text));
    println!("Part two result: {:?}", part2(&text));
}

fn part1(block: &str) -> usize {
    let (dirs, table) = parse_dirs_and_table_from_block(block);
    let mut jumps = 0;
    let mut directions = dirs.iter().cycle();
    let mut node = "AAA";
    while node != "ZZZ" {
        let direction = directions.next().unwrap();
        let (left, right) = table.get(node).unwrap();
        node = match direction {
            'R' => right,
            'L' => left,
            _ => panic!("Invalid direction!"),
        };
        jumps += 1;
    }
    jumps
}

fn part2(block: &str) -> usize {
    let (dirs, table) = parse_dirs_and_table_from_block(block);
    let directions = dirs.iter().cycle();
    let start_points: Vec<&str> = table
        .keys()
        .filter(|s| s.ends_with('A'))
        .map(|x| &x[..])
        .collect();
    start_points
        .iter()
        .map(|n| {
            let mut node = n;
            let mut dirs = directions.clone();
            let mut count = 0;
            while !node.ends_with('Z') {
                let (left, right) = table.get(node).unwrap();
                let direction = dirs.next().unwrap();
                node = match direction {
                    'R' => right,
                    'L' => left,
                    _ => panic!("Invalid path"),
                };
                count += 1;
            }
            count
        })
        .reduce(num::integer::lcm)
        .unwrap()
}

fn parse_dirs_and_table_from_block(block: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let lines: Vec<&str> = block.lines().collect();
    let directions: Vec<char> = lines.first().unwrap().chars().collect();
    let maps: HashMap<&str, (&str, &str)> =
        lines.iter().skip(2).fold(HashMap::new(), |mut map, l| {
            map.insert(&l[0..3], (&l[7..10], &l[12..15]));
            map
        });
    (directions, maps)
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT1: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const INPUT2: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn day8_part1_test1() {
        assert_eq!(part1(INPUT1), 2);
    }

    #[test]
    fn day8_part1_test2() {
        assert_eq!(part1(INPUT2), 6);
    }

    // #[test]
    // fn day7_part2_test() {
    //     assert_eq!(part2(INPUT), 5905);
    // }
}
