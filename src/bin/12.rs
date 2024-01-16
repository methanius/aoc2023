use cached::proc_macro::cached;
use std::iter::zip;

fn main() {
    let text =
        std::fs::read_to_string("data/12a.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part one result: {:?}", part1(&text));
    println!("Part two result: {:?}", part2(&text));
}

fn part2(block: &str) -> usize {
    parse_block_to_strings_et_nums(block)
        .iter()
        .map(|(s, n)| ([(*s); 5].join("?"), n.repeat(5)))
        .map(|(s, n)| count(s, n))
        .sum()
}

fn part1(block: &str) -> usize {
    parse_block_to_strings_et_nums(block)
        .iter()
        .map(|(s, n)| count((*s).to_string(), n.clone()))
        .sum()
}

#[cached]
fn count<'a>(symbols: String, nums: Vec<usize>) -> usize {
    if symbols.is_empty() {
        return usize::from(nums.is_empty());
    }

    if nums.is_empty() {
        return usize::from(!symbols.contains('#'));
    }

    let mut result = 0;

    if ".?".contains(symbols.chars().next().unwrap()) {
        result += count(symbols[1..].to_string(), nums.clone());
    }

    if "#?".contains(symbols.chars().next().unwrap()) {
        if *nums.first().unwrap() <= symbols.len()
            && !symbols[..*nums.first().unwrap()].contains('.')
            && (*nums.first().unwrap() == symbols.len()
                || symbols.chars().nth(*nums.first().unwrap()).unwrap() != '#')
        {
            let next_symbol_width = *nums.first().unwrap() + 1;
            if next_symbol_width > symbols.len() {
                result += count(String::new(), nums[1..].to_vec());
            } else {
                result += count(symbols[next_symbol_width..].to_string(), nums[1..].to_vec());
            }
        } else {
            result += 0;
        }
    }
    result
}

fn parse_block_to_strings_et_nums(block: &str) -> Vec<(&str, Vec<usize>)> {
    let nums: Vec<Vec<usize>> = block
        .lines()
        .map(|l| {
            l.split(' ')
                .last()
                .unwrap()
                .split(',')
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .collect();
    let blocks: Vec<&str> = block
        .lines()
        .map(|l| l.split(' ').next().unwrap())
        .collect();
    zip(blocks, nums).collect()
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT1: &str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn day12_part1() {
        assert_eq!(
            parse_block_to_strings_et_nums(INPUT1)
                .iter()
                .map(|(s, nums)| count(s.to_string(), nums.to_vec()))
                .collect::<Vec<usize>>(),
            vec![1, 4, 1, 1, 4, 10]
        )
    }
}
