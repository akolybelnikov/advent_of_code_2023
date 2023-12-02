use advent_of_code_2023::read_lines;

#[derive(Default)]
struct CubeSet {
    blue: i32,
    green: i32,
    red: i32,
}

impl CubeSet {
    // Create a new CubeSet from a string like: 1 red, 2 green, 6 blue
    pub fn new(str_set: &str) -> CubeSet {
        let mut set = CubeSet {
            blue: 0,
            green: 0,
            red: 0,
        };

        for color in str_set.split(", ") {
            let mut split = color.split_whitespace();
            let num = split.next().unwrap().parse::<i32>().unwrap();
            let color = split.next().unwrap();

            match color {
                "blue" => set.blue = num,
                "green" => set.green = num,
                "red" => set.red = num,
                _ => panic!("Unknown color: {}", color),
            }
        }

        set
    }

    fn power(&self) -> i32 {
        self.blue * self.green * self.red
    }
}

struct Game {
    id: i32,
    config: CubeSet,
    cube_sets: Vec<CubeSet>,
    min_set: CubeSet,
    possible: bool,
}

impl Game {
    // parse a string input "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green" into a game
    pub fn new(str_input: &str, config: &str) -> Game {
        let mut game = Game {
            id: 0,
            config: CubeSet::new(config),
            cube_sets: Vec::new(),
            min_set: CubeSet::default(),
            possible: true,
        };
        let game_split = str_input.split(":").collect::<Vec<&str>>();
        let id_split = game_split[0].split(" ").collect::<Vec<&str>>();
        game.id = match id_split[1].parse::<i32>() {
            Ok(num) => num,
            Err(_) => panic!("Invalid game id: {}", id_split[1]),
        };
        for set in game_split[1].split(";") {
            let new_set = CubeSet::new(set);
            // check if the game is possible by comparing the numbers in the cube set to the config
            if new_set.blue > game.config.blue || new_set.green > game.config.green || new_set.red > game.config.red {
                game.possible = false;
            }
            // find the smallest necessary cube set for a possible game
            if game.min_set.blue < new_set.blue {
                game.min_set.blue = new_set.blue;
            }
            if game.min_set.green < new_set.green {
                game.min_set.green = new_set.green;
            }
            if game.min_set.red < new_set.red {
                game.min_set.red = new_set.red;
            }
            game.cube_sets.push(new_set);
        }

        game
    }
}

fn part_1(filename: &str) -> i32 {
    let lines = read_lines(filename).unwrap();
    let mut total = 0;
    for line in &lines {
        let game = Game::new(line, "12 red, 13 green, 14 blue");
        if game.possible {
            total += game.id;
        }
    }
    total
}

fn part_2(filename: &str) -> i32 {
    let lines = read_lines(filename).unwrap();
    let mut sum = 0;
    for line in &lines {
        let game = Game::new(line, "12 red, 13 green, 14 blue");
        sum += game.min_set.power();
    }
    sum
}

fn main() {
    println!("Part 1: {}", part_1("src/bin/day02/input.txt"));
    println!("Part 2: {}", part_2("src/bin/day02/input.txt"));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_set_new() {
        let set = CubeSet::new(" 1 red, 2 green, 6 blue");
        assert_eq!(set.blue, 6);
        assert_eq!(set.green, 2);
        assert_eq!(set.red, 1);
    }

    #[test]
    fn test_game_new() {
        let game = Game::new("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", "1 red, 1 green, 1 blue");
        assert_eq!(game.id, 1);
        assert_eq!(game.config.blue, 1);
        assert_eq!(game.config.green, 1);
        assert_eq!(game.config.red, 1);
        assert_eq!(game.cube_sets.len(), 3);
        assert_eq!(game.cube_sets[0].blue, 3);
        assert_eq!(game.cube_sets[0].green, 0);
        assert_eq!(game.cube_sets[0].red, 4);
        assert_eq!(game.cube_sets[1].blue, 6);
        assert_eq!(game.cube_sets[1].green, 2);
        assert_eq!(game.cube_sets[1].red, 1);
        assert_eq!(game.cube_sets[2].blue, 0);
        assert_eq!(game.cube_sets[2].green, 2);
        assert_eq!(game.cube_sets[2].red, 0);
        assert_eq!(game.possible, false);
        assert_eq!(game.min_set.blue, 6);
        assert_eq!(game.min_set.green, 2);
        assert_eq!(game.min_set.red, 4);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/bin/day02/test_input.txt"), 8);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("src/bin/day02/test_input.txt"), 2286);
    }
}