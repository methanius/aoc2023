use std::ops::Div;

fn main() {
    let text =
        std::fs::read_to_string("data/10a.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part one result: {:?}", part1(&text));
    println!("Part two result: {:?}", part2(&text));
}

#[allow(clippy::cast_precision_loss)]
fn part2(block: &str) -> f64 {
    let vecblock = read_block(block);
    let start = get_start(&vecblock);
    let mut dir = find_first_direction(&vecblock, &start);
    let mut locs_in_loop: Vec<(f64, f64)> = vec![];
    let (mut cur_pip, mut loc) = go_to_next_pipe(&vecblock, &start, dir);
    locs_in_loop.push((loc.0 as f64, loc.1 as f64));
    while cur_pip != Pipe::S {
        dir = cur_pip.traverse(dir.into()).unwrap();
        (cur_pip, loc) = go_to_next_pipe(&vecblock, &loc, dir);
        locs_in_loop.push((loc.0 as f64, loc.1 as f64));
    }

    // Shoelace algorithm
    let mut twice_area: f64 = locs_in_loop
        .windows(2)
        .map(|tups| tups[0].0.mul_add(tups[1].1, -(tups[0].1 * tups[1].0)))
        .sum();
    twice_area += {
        let first = locs_in_loop.first().unwrap();
        let last = locs_in_loop.last().unwrap();
        last.0.mul_add(first.1, -(last.1 * first.0))
    };
    // Picks algorithm
    (twice_area.abs() + 2.0 - ((locs_in_loop.len()) as f64)) / 2.0
}

fn part1(block: &str) -> i64 {
    let vecblock = read_block(block);
    let start = get_start(&vecblock);
    let mut counter: i64 = 1;
    let mut dir = find_first_direction(&vecblock, &start);
    let (mut cur_pip, mut loc) = go_to_next_pipe(&vecblock, &start, dir);
    while cur_pip != Pipe::S {
        dir = cur_pip.traverse(dir.into()).unwrap();
        (cur_pip, loc) = go_to_next_pipe(&vecblock, &loc, dir);
        counter += 1;
    }
    counter.div(2)
}

fn read_block(block: &str) -> Vec<Vec<char>> {
    block.lines().map(|l| l.chars().collect()).collect()
}

fn get_start(block: &[Vec<char>]) -> (usize, usize) {
    block
        .iter()
        .enumerate()
        .find(|(_, l)| l.contains(&'S'))
        .map(|(n, l)| (n, l.iter().position(|c| c == &'S').unwrap()))
        .unwrap()
}

fn go_to_next_pipe(
    block: &[Vec<char>],
    start: &(usize, usize),
    out: Out,
) -> (Pipe, (usize, usize)) {
    let loc = out.step(start);
    let c = block[loc.0][loc.1];
    (Pipe::from_char(c).unwrap(), loc)
}

fn find_first_direction(block: &[Vec<char>], start: &(usize, usize)) -> Out {
    for direction in [
        Out(Direction::East),
        Out(Direction::South),
        Out(Direction::North),
        Out(Direction::West),
    ] {
        let loc = direction.step(start);
        let pipe = Pipe::from_char(block[loc.0][loc.1]);
        if let Some(p) = pipe {
            let out = p.traverse(direction.into());
            if out.is_some() {
                return direction;
            }
        }
    }
    panic!("There should always be a valid first direction!")
}

#[derive(Debug, Clone, Copy)]
struct Out(Direction);

impl Out {
    const fn step(self, start: &(usize, usize)) -> (usize, usize) {
        match self {
            Self(Direction::North) => (start.0 - 1, start.1),
            Self(Direction::South) => (start.0 + 1, start.1),
            Self(Direction::East) => (start.0, start.1 + 1),
            Self(Direction::West) => (start.0, start.1 - 1),
        }
    }
}

#[derive(Debug)]
struct In(Direction);

impl From<Out> for In {
    fn from(o: Out) -> Self {
        match o {
            Out(Direction::North) => Self(Direction::South),
            Out(Direction::South) => Self(Direction::North),
            Out(Direction::East) => Self(Direction::West),
            Out(Direction::West) => Self(Direction::East),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Pipe {
    Bar,
    Dash,
    L,
    J,
    Seven,
    F,
    S,
}

impl Pipe {
    const fn from_char(c: char) -> Option<Self> {
        match c {
            '|' => Some(Self::Bar),
            '-' => Some(Self::Dash),
            'L' => Some(Self::L),
            'J' => Some(Self::J),
            '7' => Some(Self::Seven),
            'F' => Some(Self::F),
            'S' => Some(Self::S),
            _ => None,
        }
    }

    const fn traverse(self, direction: In) -> Option<Out> {
        match (self, direction) {
            (Self::Bar, In(Direction::West | Direction::East))
            | (Self::Dash, In(Direction::North | Direction::South))
            | (Self::L, In(Direction::West | Direction::South))
            | (Self::J, In(Direction::South | Direction::East))
            | (Self::Seven, In(Direction::North | Direction::East))
            | (Self::F, In(Direction::North | Direction::West))
            | (Self::S, _) => None,
            (Self::Bar, In(Direction::South))
            | (Self::J, In(Direction::West))
            | (Self::L, In(Direction::East)) => Some(Out(Direction::North)),
            (Self::Bar, In(Direction::North))
            | (Self::Seven, In(Direction::West))
            | (Self::F, In(Direction::East)) => Some(Out(Direction::South)),
            (Self::Dash, In(Direction::West))
            | (Self::L, In(Direction::North))
            | (Self::F, In(Direction::South)) => Some(Out(Direction::East)),
            (Self::Dash, In(Direction::East))
            | (Self::J, In(Direction::North))
            | (Self::Seven, In(Direction::South)) => Some(Out(Direction::West)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT1: &str = "\
>.....
.S-7.
.|.|.
.L-J.
.....";

    const INPUT2: &str = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    const INPUT3: &str = "\
    ...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const INPUT4: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    const INPUT5: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
    #[test]
    fn day10_part1() {
        assert_eq!(get_start(&read_block(INPUT1)), (1, 1));
        assert_eq!(get_start(&read_block(INPUT2)), (2, 0));
        assert_eq!(part1(INPUT1), 4);
    }

    #[test]
    fn day10_part2() {
        assert_eq!(part2(INPUT1), 1.0);
        assert_eq!(part2(INPUT2), 1.0);
        assert_eq!(part2(INPUT3), 4.0);
        assert_eq!(part2(INPUT4), 8.0);
        assert_eq!(part2(INPUT5), 10.0);
    }
}
