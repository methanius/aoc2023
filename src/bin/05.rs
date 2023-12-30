use std::ops::Range;

fn main() {
    let text =
        std::fs::read_to_string("data/05a.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part one result: {:?}", part1(&text));
}

// fn part2(block: &str) -> usize {
//     let textblocks: Vec<&str> = block.split("\n\n").collect::<Vec<&str>>();
//     let mut seeds: Vec<usize> = textblocks[0]
//         .split(|s| s == ':' || s == ' ')
//         .skip(2)
//         .map(|s| s.trim().parse::<usize>().unwrap())
//         .collect();
//     let maps: Vec<StateMap> = textblocks
//         .iter()
//         .skip(1)
//         .map(|block| StateMap::block_to_state_map(block))
//         .collect();
//     *seeds
//         .iter_mut()
//         .map(|s| {
//             for trans in &maps {
//                 *s = trans.map(*s);
//             }
//             s
//         })
//         .min()
//         .unwrap()
// }

fn part1(block: &str) -> usize {
    let textblocks: Vec<&str> = block.split("\n\n").collect::<Vec<&str>>();
    let mut seeds: Vec<usize> = textblocks[0]
        .split(|s| s == ':' || s == ' ')
        .skip(2)
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect();
    let maps: Vec<StateMap> = textblocks
        .iter()
        .skip(1)
        .map(|block| StateMap::block_to_state_map(block))
        .collect();
    *seeds
        .iter_mut()
        .map(|s| {
            for trans in &maps {
                *s = trans.map(*s);
            }
            s
        })
        .min()
        .unwrap()
}

#[derive(Debug)]
struct StateMap {
    source: Vec<Range<usize>>,
    dest: Vec<Range<usize>>,
}

impl StateMap {
    fn map(&self, src: usize) -> usize {
        self.source
            .iter()
            .enumerate()
            .find(|&(_, r)| r.contains(&src))
            .map_or(src, |(c, r)| src - r.start + self.dest[c].start)
    }

    fn block_to_state_map(block: &str) -> Self {
        block
            .lines()
            .skip(1)
            .map(|s| {
                s.split(' ')
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<usize>>()
            })
            .fold(
                Self {
                    source: vec![],
                    dest: vec![],
                },
                |mut sm: Self, line: Vec<usize>| {
                    sm.source.push(line[1]..line[1] + line[2]);
                    sm.dest.push(line[0]..line[0] + line[2]);
                    sm
                },
            )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn day5_part1_test() {
        assert_eq!(part1(INPUT), 35);
    }

    // #[test]
    // fn day5_part2_test() {
    //     assert_eq!(part2(INPUT), 46);
    // }
}
