use std::ops::Div;

fn main() {
    let text =
        std::fs::read_to_string("data/10a.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part one result: {:?}", part1(&text));
    //     println!("Part two result: {:?}", part2(&text));
}

fn part1(block: &str) -> i64 {
    let vecblock = read_block(block);
    let start = get_start(&vecblock);
    let mut counter: i64 = 1;
    let mut dir = find_first_direction(&vecblock, &start);
    let (mut cur_pip, mut loc) = go_to_next_pipe(&vecblock, &start, &dir);
    while cur_pip != Pipe::S {
        dir = cur_pip.traverse(dir.into()).unwrap();
        (cur_pip, loc) = go_to_next_pipe(&vecblock, &loc, &dir);
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
    out: &Out,
) -> (Pipe, (usize, usize)) {
    let loc = out.step(start);
    let c = block[loc.0][loc.1];
    (Pipe::from_char(c).unwrap(), loc)
}

fn find_first_direction(block: &[Vec<char>], start: &(usize, usize)) -> Out {
    for direction in [
        Out(Direction::North),
        Out(Direction::East),
        Out(Direction::South),
        Out(Direction::West),
    ] {
        let loc = direction.step(start);
        let pipe = Pipe::from_char(block[loc.0][loc.1]);
        if let Some(p) = pipe {
            let out = p.traverse(direction.into());
            if let Some(a) = out {
                return a;
            }
        }
    }
    panic!("There should always be a valid first direction!")
}

#[derive(Debug)]
struct Out(Direction);

impl Out {
    const fn step(&self, start: &(usize, usize)) -> (usize, usize) {
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

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(PartialEq, Debug)]
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

    const fn traverse(&self, direction: In) -> Option<Out> {
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

    #[test]
    fn day10_part1() {
        assert_eq!(get_start(&read_block(INPUT1)), (1, 1));
        assert_eq!(get_start(&read_block(INPUT2)), (2, 0));
        assert_eq!(part1(INPUT1), 4);
    }
}
