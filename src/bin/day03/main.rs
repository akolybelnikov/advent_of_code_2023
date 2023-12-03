// --- Day 3: Gear Ratios ---
struct Number {
    value: i32,
    adjacent: Vec<(usize, usize)>,
}

impl Number {
    fn from_string(s: &str) -> Number {
        Number {
            value: s.parse::<i32>().unwrap(),
            adjacent: vec![],
        }
    }
}


fn main() {
    println!("Part 1: {}", part_1("src/bin/day03/input.txt"));
}

fn part_1(filename: &str) -> i32 {
    let mut sum = 0;
    let lines = advent_of_code_2023::read_lines(filename).unwrap();
    let mut numbers = Vec::new();
    let mut symbols: Vec<(usize, usize)> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        let mut chars = line.chars().collect::<Vec<_>>();
        chars.push(' ');
        let mut i = 0;
        while i < chars.len() - 1 {
            if chars[i].is_digit(10) {
                i = process_number(&chars, i, y, line, &lines, &mut numbers);
            } else if chars[i] != '.' {
                symbols.push((y, i));
            }
            i += 1;
        }
    }
    for num in &numbers {
        sum_part_numbers(num, &symbols, &mut sum);
    }
    sum
}

fn process_number(chars: &[char], i: usize, y: usize, line: &str, lines: &[String], numbers: &mut Vec<Number>) -> usize {
    let start = i;
    let mut i = start;
    while chars[i].is_digit(10) {
        i += 1;
    }
    let end = i - 1;
    let digits: String = chars[start..=end].iter().collect();
    let mut number = Number::from_string(&digits);
    let x1 = if start > 0 {
        number.adjacent.push((y, start - 1));
        start - 1
    } else { 0 };
    let x2 = if end < line.len() - 1 {
        number.adjacent.push((y, end + 1));
        end + 1
    } else { line.len() - 1 };
    for i in x1..=x2 {
        if y > 0 {
            number.adjacent.push((y - 1, i));
        }
        if y < lines.len() - 1 {
            number.adjacent.push((y + 1, i));
        }
    }
    numbers.push(number);
    end
}

fn sum_part_numbers(num: &Number, symbols: &[(usize, usize)], sum: &mut i32) {
    for pos in &num.adjacent {
        if symbols.contains(pos) {
            *sum += num.value;
            break;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/bin/day03/test_input_1.txt"), 4361);
    }
}
