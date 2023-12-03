// --- Day 2: Cube Conundrum ---
use std::collections::HashMap;
use advent_of_code_2023::read_lines;

struct CubeSet {
    cubes: HashMap<String, i32>,
}

const BLUE: &str = "blue";
const GREEN: &str = "green";
const RED: &str = "red";
const COLORS: [&str; 3] = [BLUE, GREEN, RED];

impl Default for CubeSet {
    fn default() -> Self {
        CubeSet {
            cubes: COLORS.iter().map(|&color| (color.to_string(), 0)).collect(),
        }
    }
}

impl CubeSet {
    // Create a new CubeSet from a string like: 1 red, 2 green, 6 blue
    pub fn new(str_set: &str) -> CubeSet {
        let mut set = CubeSet::default();

        for color_and_num in str_set.split(",") {
            let (num, color) = CubeSet::parse_color_and_num(color_and_num);
            set.cubes.insert(color, num);
        }

        set
    }

    fn parse_color_and_num(color_and_num: &str) -> (i32, String) {
        let mut split = color_and_num.split_whitespace();
        let num = split.next().unwrap().parse::<i32>().unwrap();
        let color = split.next().unwrap().to_string();
        (num, color)
    }

    fn power(&self) -> i32 {
        self.cubes.values().product()
    }

    fn blue(&self) -> i32 {
        self.cubes[BLUE]
    }

    fn green(&self) -> i32 {
        self.cubes[GREEN]
    }

    fn red(&self) -> i32 {
        self.cubes[RED]
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
    /// Parses a string input and a config into a Game struct.
    ///
    /// # Arguments
    ///
    /// * `str_input` - A string input in the format "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    /// * `config` - A string representing the configuration of the game
    ///
    /// # Returns
    ///
    /// A Game struct with the parsed id, config, cube sets, minimum set, and possibility status.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::Game;
    ///
    /// let str_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    /// let config = "1 blue, 12 green, 6 red";
    /// let game = Game::new(str_input, config);
    /// ```
    pub fn new(str_input: &str, config: &str) -> Game {
        let mut game = Game {
            id: 0,
            config: CubeSet::new(config),
            cube_sets: Vec::new(),
            min_set: CubeSet::default(),
            possible: true,
        };

        let game_split = str_input.split(":").collect::<Vec<&str>>();

        game.id = Game::parse_id(game_split[0]);

        for set in game_split[1].split(";") {
            game.determine_possibility_and_minimum_set(CubeSet::new(set));
        }

        game
    }

    fn parse_id(str_id: &str) -> i32 {
        let id_split = str_id.split(" ").collect::<Vec<&str>>();
        match id_split[1].parse::<i32>() {
            Ok(num) => num,
            Err(_) => panic!("Invalid game id: {}", id_split[1]),
        }
    }

    fn determine_possibility_and_minimum_set(&mut self, cubes: CubeSet) {
        // check if the game is possible by comparing the numbers in the cube set to the config
        if cubes.blue() > self.config.blue() || cubes.green() > self.config.green() || cubes.red() > self.config.red() {
            self.possible = false;
        }
        // find the smallest necessary cube set for a possible game
        if self.min_set.blue() < cubes.blue() {
            self.min_set.cubes.insert(BLUE.to_string(), cubes.blue());
        }
        if self.min_set.green() < cubes.green() {
            self.min_set.cubes.insert(GREEN.to_string(), cubes.green());
        }
        if self.min_set.red() < cubes.red() {
            self.min_set.cubes.insert(RED.to_string(), cubes.red());
        }
        self.cube_sets.push(cubes);
    }
}

fn part_1(filename: &str, config: &str) -> i32 {
    let lines = read_lines(filename).unwrap();
    let mut total = 0;
    for line in &lines {
        let game = Game::new(line, config);
        if game.possible {
            total += game.id;
        }
    }
    total
}

fn part_2(filename: &str, config: &str) -> i32 {
    let lines = read_lines(filename).unwrap();
    let mut sum = 0;
    for line in &lines {
        let game = Game::new(line, config);
        sum += game.min_set.power();
    }
    sum
}

fn main() {
    let config = "12 red, 13 green, 14 blue";
    println!("Part 1: {}", part_1("src/bin/day02/input.txt", config));
    println!("Part 2: {}", part_2("src/bin/day02/input.txt", config));
}


#[cfg(test)]
mod tests {
    use super::*;

    const CONFIG: &str = "12 red, 13 green, 14 blue";
    const CUBE_SET: &str = "  1 red, 2 green,    6 blue";
    const GAME: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    const GAME_CONFIG: &str = "1 red, 1 green, 1 blue";
    const TEST_FILENAME: &str = "src/bin/day02/test_input.txt";

    #[test]
    fn test_cube_set_new() {
        let set = CubeSet::new(CUBE_SET);
        assert_eq!(set.blue(), 6);
        assert_eq!(set.green(), 2);
        assert_eq!(set.red(), 1);
    }

    #[test]
    fn test_game_new() {
        let game = Game::new(GAME, GAME_CONFIG);
        assert_eq!(game.id, 1);
        assert_eq!(game.config.blue(), 1);
        assert_eq!(game.config.green(), 1);
        assert_eq!(game.config.red(), 1);
        assert_eq!(game.cube_sets.len(), 3);
        assert_eq!(game.cube_sets[0].blue(), 3);
        assert_eq!(game.cube_sets[0].green(), 0);
        assert_eq!(game.cube_sets[0].red(), 4);
        assert_eq!(game.cube_sets[1].blue(), 6);
        assert_eq!(game.cube_sets[1].green(), 2);
        assert_eq!(game.cube_sets[1].red(), 1);
        assert_eq!(game.cube_sets[2].blue(), 0);
        assert_eq!(game.cube_sets[2].green(), 2);
        assert_eq!(game.cube_sets[2].red(), 0);
        assert_eq!(game.possible, false);
        assert_eq!(game.min_set.blue(), 6);
        assert_eq!(game.min_set.green(), 2);
        assert_eq!(game.min_set.red(), 4);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_FILENAME, CONFIG), 8);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_FILENAME, CONFIG), 2286);
    }
}