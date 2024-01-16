fn main() {
    let text =
        std::fs::read_to_string("data/11a.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part one result: {:?}", part1(&text));
    println!("Part two result: {:?}", part2(&text));
}

fn part2(block: &str) -> i64 {
    let stars = expand_empty_space(&parse_block_to_star_pairs(block), 999_999);
    stars
        .iter()
        .enumerate()
        .flat_map(|(n, star)| {
            stars[(n + 1)..]
                .iter()
                .map(|(row, col)| (row - star.0).abs() + (col - star.1).abs())
                .collect::<Vec<i64>>()
        })
        .sum()
}

fn part1(block: &str) -> i64 {
    let stars = expand_empty_space(&parse_block_to_star_pairs(block), 1);
    stars
        .iter()
        .enumerate()
        .flat_map(|(n, star)| {
            stars[(n + 1)..]
                .iter()
                .map(|(row, col)| (row - star.0).abs() + (col - star.1).abs())
                .collect::<Vec<i64>>()
        })
        .sum()
}

#[allow(clippy::cast_possible_wrap)]
fn parse_block_to_star_pairs(block: &str) -> Vec<(i64, i64)> {
    block
        .lines()
        .enumerate()
        .flat_map(|(n, c)| {
            let mut res = vec![];
            for (m, c) in c.chars().enumerate() {
                if c == '#' {
                    res.push((n as i64, m as i64));
                }
            }
            res
        })
        .collect()
}

fn expand_empty_space(stars: &[(i64, i64)], expansion: i64) -> Vec<(i64, i64)> {
    let max_row = *stars.iter().map(|(row, _)| row).max().unwrap();
    let max_col = *stars.iter().map(|(_, col)| col).max().unwrap();
    let mut new_stars: Vec<(i64, i64)> = stars.to_vec();
    for n in (0..max_row).rev() {
        if stars
            .iter()
            .map(|(row, _)| row)
            .filter(|row| **row == n)
            .count()
            == 0
        {
            new_stars = new_stars
                .into_iter()
                .map(|(row, col)| {
                    if row > n {
                        (row + expansion, col)
                    } else {
                        (row, col)
                    }
                })
                .collect();
        }
    }
    for n in (0..max_col).rev() {
        if stars
            .iter()
            .map(|(_, col)| col)
            .filter(|col| **col == n)
            .count()
            == 0
        {
            new_stars = new_stars
                .into_iter()
                .map(|(row, col)| {
                    if col > n {
                        (row, col + expansion)
                    } else {
                        (row, col)
                    }
                })
                .collect();
        }
    }
    new_stars
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT1: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn day11_part1() {
        let pre_expansion_coordinates: Vec<(i64, i64)> = parse_block_to_star_pairs(INPUT1);
        assert_eq!(
            pre_expansion_coordinates,
            vec![
                (0, 3),
                (1, 7),
                (2, 0),
                (4, 6),
                (5, 1),
                (6, 9),
                (8, 7),
                (9, 0),
                (9, 4)
            ]
        );
        assert_eq!(
            expand_empty_space(&pre_expansion_coordinates, 1),
            vec![
                (0, 4),
                (1, 9),
                (2, 0),
                (5, 8),
                (6, 1),
                (7, 12),
                (10, 9),
                (11, 0),
                (11, 5)
            ]
        );
        assert_eq!(part1(INPUT1), 374)
    }

    // #[test]
    // fn day10_part2() {
    // }
}
