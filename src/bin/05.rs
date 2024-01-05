use std::ops::Range;

fn main() {
    let text =
        std::fs::read_to_string("data/05a.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part one result: {:?}", part1(&text));
    println!("Part two result: {:?}", part2(&text));
}

fn part2(block: &str) -> usize {
    let textblocks: Vec<&str> = block.split("\n\n").collect::<Vec<&str>>();
    let seed_numbers: Vec<(usize, usize)> = textblocks[0]
        .split(|s| s == ':' || s == ' ')
        .skip(2)
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .chunks(2)
        .map(|a| (a[0], a[1]))
        .collect();
    let maps: Vec<StateMap> = textblocks
        .iter()
        .skip(1)
        .map(|block| StateMap::block_to_state_map(block))
        .collect();
    let mut state = seed_numbers;
    for map in maps {
        state = map.map_tuple_vector(state);
    }
    state.iter().min_by_key(|r| r.0).unwrap().0
}

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

    fn map_tuple_vector(&self, src: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut ret = vec![];
        for rangetuple in src {
            let mut start = rangetuple.0;
            let stop = rangetuple.0 + rangetuple.1;
            while start != stop {
                match self.source.iter().find(|r| r.contains(&start)) {
                    Some(r) => {
                        if r.contains(&stop) {
                            ret.push((self.map(start), self.map(stop) - self.map(start)));
                            start = stop;
                        } else {
                            ret.push((self.map(start), self.map(r.end - 1) - self.map(start)));
                            start = if r.end == stop { stop } else { r.end };
                        }
                    }
                    None => {
                        if let Some(r) = self
                            .source
                            .iter()
                            .filter(|r| start < r.start && stop >= r.start)
                            .min_by_key(|r| r.start - start)
                        {
                            ret.push((start, r.start - start));
                            start = r.start;
                        } else {
                            ret.push((start, stop - start));
                            start = stop;
                        }
                    }
                }
            }
        }
        ret
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

    #[test]
    fn day5_part2_test() {
        assert_eq!(part2(INPUT), 46);
    }
}
