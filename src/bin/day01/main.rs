use advent_of_code_2023::read_lines;

fn main() {
    println!("Part 1: {}", part_1());
}

fn part_1() -> i32 {
    let lines = read_lines("src/bin/day01/input.txt").unwrap();
    let mut total = 0;
    for line in &lines {
        if let Some(number) = get_line_number(line) {
            total += number;
        }
    }
    total
}

fn get_line_number(line: &str) -> Option<i32> {
    let digits = line.chars().filter(|c| c.is_digit(10)).collect::<Vec<char>>();
    if digits.is_empty() {
        None
    } else {
        let mut number = String::new();
        number.push (digits[0]);
        number.push (*digits.last().unwrap());
        Some(number.parse::<i32>().unwrap())
    }
}