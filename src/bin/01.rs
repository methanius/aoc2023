fn numbers_from_line(line: &str) -> u32 {
    let mut digits = line.chars().filter_map(|c| c.to_digit(10));
    let first = digits.next().unwrap();
    let last = digits.last().unwrap_or(first);
    10 * first + last
}

fn part_1() {
    let text =
        std::fs::read_to_string("data/01a.txt").expect("Couldn't read file at hard-coded path!");
    let sum: u32 = text.lines().map(numbers_from_line).sum();
    println!("Sum: {sum}");
}

fn main() {
    part_1();
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_01_numbers_vector() {
        let line = INPUT.lines().next().map(numbers_from_line);
        assert_eq!(line, Some(12));
        let numbers_vector = INPUT.lines().map(numbers_from_line).collect::<Vec<u32>>();
        assert_eq!(vec![12, 38, 15, 77], numbers_vector);
        let sum = numbers_vector.iter().sum::<u32>();
        assert_eq!(sum, 142);
    }
}
