fn main() {
    let text =
        std::fs::read_to_string("data/03a.txt").expect("Couldn't read file at hard-coded path!");
    println!(
        "Part 1 has answer {:?}",
        part1_number_node_conditional_sum(&text)
    );
    println!("Part 2 has answer {:?}", part_2(&text));
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

fn locate_gears_in_lines(block: &str) -> Vec<(usize, usize)> {
    block
        .lines()
        .enumerate()
        .flat_map(|(i, s)| {
            s.chars()
                .enumerate()
                .filter(|(_, c)| c == &'*')
                .map(move |(n, _)| (i, n))
        })
        .collect()
}

fn gear_number_neighbours_from_locations(
    locations: &[(usize, usize)],
    block: &str,
) -> Vec<Vec<u32>> {
    let vec_block = block
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let num_rows = vec_block.len();
    let num_cols = vec_block[0].len();
    let mut numbers_vector: Vec<Vec<u32>> = vec![];
    for coord_pair in locations {
        let mut number_neighbours: Vec<u32> = vec![];
        let mut coords_searched: Vec<(usize, usize)> = vec![*coord_pair];
        let start_row = coord_pair.0.saturating_sub(1);
        let stop_row = if coord_pair.0 + 1 < num_rows {
            coord_pair.0 + 1
        } else {
            coord_pair.0
        };
        let start_col = coord_pair.1.saturating_sub(1);
        let stop_col = if coord_pair.1 + 1 < num_cols {
            coord_pair.1 + 1
        } else {
            coord_pair.1
        };
        for (row, block_row) in vec_block
            .iter()
            .enumerate()
            .take(stop_row + 1)
            .skip(start_row)
        {
            for (col, c) in block_row
                .iter()
                .enumerate()
                .take(stop_col + 1)
                .skip(start_col)
            {
                if !coords_searched.contains(&(row, col)) {
                    coords_searched.push((row, col));
                    if c.is_digit(INT_RADIX) {
                        let mut value: u32 = c.to_digit(INT_RADIX).unwrap();
                        let mut new_col = col.saturating_sub(1);
                        let mut shift = 10;
                        while !coords_searched.contains(&(row, new_col))
                            && vec_block[row][new_col].is_digit(INT_RADIX)
                        {
                            coords_searched.push((row, new_col));
                            value += vec_block[row][new_col].to_digit(INT_RADIX).unwrap() * shift;
                            shift *= shift;
                            new_col = new_col.saturating_sub(1);
                        }
                        new_col = if col + 1 < num_cols { col + 1 } else { col };
                        shift = 10;
                        while !coords_searched.contains(&(row, new_col))
                            && vec_block[row][new_col].is_digit(INT_RADIX)
                        {
                            coords_searched.push((row, new_col));
                            value *= shift;
                            value += vec_block[row][new_col].to_digit(INT_RADIX).unwrap();
                            new_col = if new_col + 1 < num_cols {
                                new_col + 1
                            } else {
                                new_col
                            };
                        }
                        number_neighbours.push(value);
                    }
                }
            }
        }
        numbers_vector.push(number_neighbours);
    }
    numbers_vector
}

fn part_2(block: &str) -> u32 {
    gear_number_neighbours_from_locations(&locate_gears_in_lines(block), block)
        .into_iter()
        .filter(|v| v.len() == 2)
        .map(|v| {
            v.into_iter()
                .reduce(|acc, e| acc * e)
                .expect("vectors of only u32 must be reducable this way disregarding overflow")
        })
        .sum()
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

    #[test]
    fn find_gears() {
        assert_eq!(locate_gears_in_lines(&INPUT), vec![(1, 3), (4, 3), (8, 5)]);
    }

    #[test]
    fn gear_neighbours() {
        assert_eq!(
            gear_number_neighbours_from_locations(&locate_gears_in_lines(&INPUT), &INPUT),
            vec![vec![467, 35], vec![617], vec![755, 598]]
        );
    }

    #[test]
    fn day3_part_2_test_input() {
        assert_eq!(part_2(INPUT), 467835);
    }
}
