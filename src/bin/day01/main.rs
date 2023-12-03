// --- Day 1: Trebuchet?! ---
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
    let digits = line.chars().filter(|c| c.is_digit(10)).
        collect::<Vec<char>>();

    digits.first().zip(digits.last()).and_then(|(first, last)| {
        let mut number = String::new();
        number.push(*first);
        number.push(*last);
        number.parse::<i32>().ok()
    })
}

fn part_2(filename: &str) -> i32 {
    let lines = read_lines(filename).unwrap();
    let mut total = 0;
    let patterns = vec!["one", "two", "three", "four", "five", "six", "seven", "eight",
                        "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let regexes: Vec<Regex> = patterns.iter().map(|x| Regex::new(x).unwrap()).collect();

    for line in &lines {
        if let Some(number) = get_line_number_2(line, &regexes) {
            total += number;
        }
    }
    total
}

/// This function searches for occurrences of regular expressions in a given line.
/// It returns the i32 number corresponding to the first and last match found, if any.
///
/// # Arguments
///
/// * `line` - A string slice representing the line to search for matches.
/// * `regexes` - A vector of regular expressions to use for matching, in our case the words for
/// numbers from 1 to 9 and the digits 1 to 9.
///
/// # Returns
///
/// An optional integer representing the line number, or `None` if no matches are found.
/// ```
fn get_line_number_2(line: &str, regexes: &Vec<Regex>) -> Option<i32> {
    let mut matches: Vec<(i32, &str)> = Vec::new();
    for re in regexes {
        for mat in re.find_iter(line) {
            matches.push((mat.start() as i32, &mat.as_str()));
        }
    }
    if matches.len() > 0 {
        matches.sort_by(|a, b| a.0.cmp(&b.0));
        let number = match_number(String::new(), matches[0].1);
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
    fn test_get_line_number_2() {
        let patterns = vec!["one", "two", "three", "four", "five", "six", "seven", "eight",
                            "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
        let regexes: Vec<Regex> = patterns.iter().map(|x| Regex::new(x).unwrap()).collect();
        assert_eq!(get_line_number_2("yytwo1tt6three4", &regexes), Some(24));
        assert_eq!(get_line_number_2("1twpoo43", &regexes), Some(13));
        assert_eq!(get_line_number_2("fffsix10fghfutwo2", &regexes), Some(62));
        assert_eq!(get_line_number_2("ddd1four", &regexes), Some(14));
        assert_eq!(get_line_number_2("vvv", &regexes), None);
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