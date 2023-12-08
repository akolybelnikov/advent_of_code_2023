// --- Day 6: Wait For It ---
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn count_wins(&self) -> u64 {
        let mut rising = true;
        let mut count = 0;
        let mut presstime = 0;
        let mut cur = 0;
        while rising {
            presstime += 1;
            let dist = presstime * (self.time - presstime);
            if dist > cur || dist > self.distance {
                cur = dist;
            } else {
                rising = false;
            }
            if dist > self.distance {
                count += 1;
            }
        }
        count
    }
}

fn process_race_line(line: &str) -> Vec<u64> {
    line.split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn parse_races(lines: Vec<String>) -> Vec<Race> {
    let times: Vec<u64> = process_race_line(&lines[0]);
    let distances: Vec<u64> = process_race_line(&lines[1]);
    let races = times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| Race { time, distance })
        .collect::<Vec<Race>>();

    races
}

fn process_race_line_num(line: &str) -> u64 {
    let cleaned: String = line
        .split(":")
        .nth(1)
        .unwrap()
        .chars()
        .filter(|ch| ch.is_digit(10))
        .collect();
    cleaned.parse().unwrap()
}

fn parse_race(lines: Vec<String>) -> Race {
    let time = process_race_line_num(&lines[0]);
    let distance = process_race_line_num(&lines[1]);

    Race { time, distance }
}

fn part_1(filename: &str) -> u64 {
    let mut count: u64 = 1;
    let lines = advent_of_code_2023::read_lines(filename).unwrap();
    let races = parse_races(lines);
    for race in races {
        count *= race.count_wins();
    }
    count
}

fn part_2(filename: &str) -> u64 {
    let race = parse_race(advent_of_code_2023::read_lines(filename).unwrap());
    race.count_wins()
}

fn main() {
    println!("{:?}", part_1("src/bin/day06/input.txt"));
    println!("{:?}", part_2("src/bin/day06/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_races() {
        let lines = advent_of_code_2023::read_lines("src/bin/day06/test_input.txt");
        let races = parse_races(lines.unwrap());
        assert_eq!(races.len(), 3);
        assert_eq!(races[0].time, 7);
        assert_eq!(races[0].distance, 9);
        assert_eq!(races[1].time, 15);
        assert_eq!(races[1].distance, 40);
        assert_eq!(races[2].time, 30);
        assert_eq!(races[2].distance, 200);
    }

    #[test]
    fn test_count_wins() {
        let lines = advent_of_code_2023::read_lines("src/bin/day06/test_input.txt");
        let races = parse_races(lines.unwrap());
        assert_eq!(races[0].count_wins(), 4);
        assert_eq!(races[1].count_wins(), 8);
        assert_eq!(races[2].count_wins(), 9);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/bin/day06/test_input.txt"), 288);
    }

    #[test]
    fn test_parse_race() {
        let lines = advent_of_code_2023::read_lines("src/bin/day06/test_input.txt");
        let race = parse_race(lines.unwrap());
        assert_eq!(race.time, 71530);
        assert_eq!(race.distance, 940200);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("src/bin/day06/test_input.txt"), 71503);
    }
}
