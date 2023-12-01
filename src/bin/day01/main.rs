use advent_of_code_2023::read_lines;
use regex::Regex;

fn main() {
    println!("Part 1: {}", part_1("src/bin/day01/input.txt"));
    println!("Part 2: {}", part_2("src/bin/day01/input.txt"));
}

fn part_1(filename: &str) -> i32 {
    let lines = read_lines(filename).unwrap();
    let mut total = 0;
    for line in &lines {
        if let Some(number) = get_line_number_1(line) {
            total += number;
        }
    }
    total
}

fn get_line_number_1(line: &str) -> Option<i32> {
    let digits = line.chars().filter(|c| c.is_digit(10)).collect::<Vec<char>>();
    if digits.is_empty() {
        None
    } else {
        let mut number = String::new();
        number.push(digits[0]);
        number.push(*digits.last().unwrap());
        Some(number.parse::<i32>().unwrap())
    }
}

fn part_2(filename: &str) -> i32 {
    let lines = read_lines(filename).unwrap();
    let mut total = 0;
    for line in &lines {
        if let Some(number) = get_line_number_2(line) {
            total += number;
        }
    }
    total
}

fn get_line_number_2(line: &str) -> Option<i32> {
    let patterns = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut matches: Vec<(i32, &str)> = Vec::new();
    for pattern in &patterns {
        let re = Regex::new(pattern).unwrap();
        for mat in re.find_iter(line) {
            matches.push((mat.start() as i32, pattern));
        }
    }
    if matches.len() > 0 {
        matches.sort_by(|a, b| a.0.cmp(&b.0));
        let number = String::new();
        let number = match_number(number, matches[0].1);
        let number = match_number(number, matches[matches.len() - 1].1);
        Some(number.parse::<i32>().unwrap())
    } else {
        None
    }
}

fn match_number(mut number: String, mat: &str) -> String {
    match mat {
        "one" => number.push('1'),
        "two" => number.push('2'),
        "three" => number.push('3'),
        "four" => number.push('4'),
        "five" => number.push('5'),
        "six" => number.push('6'),
        "seven" => number.push('7'),
        "eight" => number.push('8'),
        "nine" => number.push('9'),
        _ => number.push_str(mat),
    }
    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_line_number_1() {
        assert_eq!(get_line_number_1("yy1twott6three4"), Some(14));
        assert_eq!(get_line_number_1("1twpoo43"), Some(13));
        assert_eq!(get_line_number_1("fff10fghfutwo2"), Some(12));
        assert_eq!(get_line_number_1("ddd1f"), Some(11));
        assert_eq!(get_line_number_1("vvv"), None);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/bin/day01/test_input_1.txt"), 142);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("src/bin/day01/test_input_2.txt"), 281);
    }
}