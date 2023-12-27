fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let text =
        std::fs::read_to_string("data/02a.txt").expect("Couldn't read file at hard-coded path!");
    let part_one_sum: u32 = text
        .lines()
        .filter_map(|l| {
            parse_winning_games(
                l,
                &CubeCounts {
                    red: 12,
                    green: 13,
                    blue: 14,
                },
            )
        })
        .sum();
    println!("Sum of winning indices is {part_one_sum}");
}

fn part_2() {
    let text =
        std::fs::read_to_string("data/02a.txt").expect("Couldn't read file at hard-coded path!");
    let part_two_sum: u32 = text.lines().map(parse_game_power).sum();
    println!("Sum of game powers is {part_two_sum}");
}

fn parse_winning_games(line: &str, comparison: &CubeCounts) -> Option<u32> {
    let (game_id, max_cube_counts) = parse_game_to_index_et_max_cube_of_each_color(line);
    if CubeCounts::any_element_larger_in_b_than_a(comparison, &max_cube_counts) {
        None
    } else {
        Some(game_id)
    }
}

fn parse_game_power(line: &str) -> u32 {
    let list_of_cube_counts = line.split(':').collect::<Vec<_>>()[1]
        .split(';')
        .flat_map(|s| s.split(','))
        .map(str::trim)
        .map(|s| s.split(' ').collect::<Vec<_>>())
        .map(|l| CubeCounts::cube_count_from_string_and_int(l[1], l[0].parse().unwrap()).unwrap())
        .collect::<Vec<CubeCounts>>();
    CubeCounts::power_of_elements(&max_cube_counts_seen(&list_of_cube_counts))
}

fn parse_game_to_index_et_max_cube_of_each_color(line: &str) -> (u32, CubeCounts) {
    let game_counts_split: Vec<_> = line.split(':').collect();
    let game_id: u32 = game_counts_split[0]
        .split(' ')
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let cube_counts = game_counts_split[1]
        .split(';')
        .flat_map(|s| s.split(','))
        .map(str::trim)
        .map(|s| s.split(' ').collect::<Vec<_>>())
        .map(|l| CubeCounts::cube_count_from_string_and_int(l[1], l[0].parse().unwrap()).unwrap())
        .collect::<Vec<_>>();
    (game_id, max_cube_counts_seen(&cube_counts))
    // CubeCounts { green: 0, blue: 0, red: 0 }
}

fn max_cube_counts_seen(cubes_seen: &[CubeCounts]) -> CubeCounts {
    let max_green = cubes_seen.iter().max_by_key(|cc| cc.green).unwrap();
    let max_blue = cubes_seen.iter().max_by_key(|cc| cc.blue).unwrap();
    let max_red = cubes_seen.iter().max_by_key(|cc| cc.red).unwrap();
    CubeCounts {
        red: max_red.red,
        blue: max_blue.blue,
        green: max_green.green,
    }
}

#[derive(Debug, PartialEq)]
struct CubeCounts {
    green: u32,
    blue: u32,
    red: u32,
}

impl CubeCounts {
    fn cube_count_from_string_and_int(name: &str, count: u32) -> Option<Self> {
        match name {
            "green" => Some(Self {
                green: count,
                red: 0,
                blue: 0,
            }),
            "blue" => Some(Self {
                green: 0,
                red: 0,
                blue: count,
            }),
            "red" => Some(Self {
                green: 0,
                red: count,
                blue: 0,
            }),
            _ => None,
        }
    }

    const fn any_element_larger_in_b_than_a(a: &Self, b: &Self) -> bool {
        matches!((a, b), _ if (b.red > a.red || b.blue > a.blue || b.green > a.green))
    }

    const fn power_of_elements(a: &Self) -> u32 {
        a.red * a.blue * a.green
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    #[test]
    fn test_parse_cube_counts_and_id() {
        assert_eq!(
            INPUT
                .lines()
                .next()
                .map(parse_game_to_index_et_max_cube_of_each_color)
                .unwrap(),
            (
                1,
                CubeCounts {
                    green: 2,
                    blue: 6,
                    red: 4
                }
            )
        );
    }

    #[test]
    fn test_filter_impossible_games() {
        let winning_indeces_sum: u32 = INPUT
            .lines()
            .filter_map(|l| {
                parse_winning_games(
                    &l,
                    &CubeCounts {
                        green: 13,
                        blue: 14,
                        red: 12,
                    },
                )
            })
            .sum();
        assert_eq!(winning_indeces_sum, 8);
    }

    const INPUT2: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_power_of_test_games() {
        let power: u32 = INPUT2.lines().map(parse_game_power).sum();
        assert_eq!(power, 2286);
    }
}
