fn main() {
    let text =
        std::fs::read_to_string("data/03a.txt").expect("Couldn't read file at hard-coded path!");
    println!(
        "Part 1 has answer {:?}",
        part1_number_node_conditional_sum(&text)
    );
}

const INT_RADIX: u32 = 10;

fn part1_number_node_conditional_sum(block: &str) -> u32 {
    block
        .lines()
        .enumerate()
        .map(|(i, l)| {
            line_to_numbers_and_coordinates(l)
                .into_iter()
                .map(|v| (i, v))
                .collect::<Vec<(usize, IndexedNumber)>>()
        })
        .flat_map(|ind_num| {
            index_numbers_and_block_to_number_nodes(&ind_num, &string_block_to_vec_vec_char(block))
        })
        .filter(NumberNodes::is_active)
        .map(|node| node.value)
        .sum()
}

fn string_block_to_vec_vec_char(block: &str) -> Vec<Vec<char>> {
    block
        .lines()
        .map(|l| l.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn index_numbers_and_block_to_number_nodes(
    ind_nums: &[(usize, IndexedNumber)],
    block: &Vec<Vec<char>>,
) -> Vec<NumberNodes> {
    // Assuming square block
    let num_rows = block.len();
    let num_cols = block[0].len();
    ind_nums
        .iter()
        .map(|(i, ind_num)| {
            let mut neighbour_indeces: Vec<(usize, usize)> = vec![];
            let start_row = i.saturating_sub(1);
            let stop_row = if i + 1 < num_rows { *i + 1 } else { *i };
            let start_col = ind_num.coordinates.0.saturating_sub(1);
            let stop_col = if ind_num.coordinates.1 + 1 < num_cols {
                ind_num.coordinates.1 + 1
            } else {
                ind_num.coordinates.1
            };
            for row in start_row..=stop_row {
                for col in start_col..=stop_col {
                    if !(row == *i && { ind_num.coordinates.0..=ind_num.coordinates.1 }
                        .contains(&col))
                    {
                        neighbour_indeces.push((row, col));
                    }
                }
            }
            (ind_num.value, neighbour_indeces)
        })
        .map(|(value, ind_vecs)| {
            (
                value,
                ind_vecs
                    .into_iter()
                    .map(|(row, col)| block[row][col])
                    .collect::<Vec<char>>(),
            )
        })
        .map(|(value, neighbour_chars)| NumberNodes {
            value,
            neighbours: neighbour_chars,
        })
        .collect::<Vec<NumberNodes>>()
}

fn line_to_numbers_and_coordinates(line: &str) -> Vec<IndexedNumber> {
    let index_value_pair: Vec<_> = line
        .trim()
        .chars()
        .enumerate()
        .filter_map(|(i, c)| {
            if c.is_digit(INT_RADIX) {
                Some((
                    i,
                    c.to_digit(INT_RADIX).expect("This should be guaranteed."),
                ))
            } else {
                None
            }
        })
        .collect();
    let mut result: Vec<IndexedNumber> = vec![];
    let mut current_number_node: Option<IndexedNumber> = None;
    for index_value in &index_value_pair {
        current_number_node = match (current_number_node, index_value) {
            (None, (index, value)) => Some(IndexedNumber {
                value: *value,
                coordinates: (*index, *index),
            }),
            (Some(node), (index, value)) => {
                if *index == node.coordinates.1 + 1 {
                    Some(IndexedNumber {
                        value: node.value * 10 + value,
                        coordinates: (node.coordinates.0, *index),
                    })
                } else {
                    result.push(node);
                    Some(IndexedNumber {
                        value: *value,
                        coordinates: (*index, *index),
                    })
                }
            }
        }
    }
    if let Some(node) = current_number_node {
        result.push(node);
    }
    result
}

#[derive(Debug, Clone, Copy)]
struct IndexedNumber {
    value: u32,
    coordinates: (usize, usize),
}

struct NumberNodes {
    value: u32,
    neighbours: Vec<char>,
}

impl NumberNodes {
    fn is_active(&self) -> bool {
        self.neighbours
            .iter()
            .any(|c| c != &'.' && !c.is_digit(INT_RADIX))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";
    #[test]
    fn day3_part1_test_input() {
        assert_eq!(part1_number_node_conditional_sum(INPUT), 4361);
    }
}
