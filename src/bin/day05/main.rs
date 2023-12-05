#[derive(Default, Debug)]
struct Almanac {
    seeds: Vec<u64>,
    soil: Vec<[u64; 3]>,
    fertilizer: Vec<[u64; 3]>,
    water: Vec<[u64; 3]>,
    light: Vec<[u64; 3]>,
    temperature: Vec<[u64; 3]>,
    humidity: Vec<[u64; 3]>,
    location: Vec<[u64; 3]>,
}

impl Almanac {
    fn new(filename: &str) -> Almanac {
        let (seeds, maps) = parse_maps(filename);
        let mut almanac = Almanac::default();
        almanac.seeds = seeds;
        for (map_name, map) in maps {
            match map_name.as_str() {
                "seed-to-soil map:" => almanac.soil = map,
                "soil-to-fertilizer map:" => almanac.fertilizer = map,
                "fertilizer-to-water map:" => almanac.water = map,
                "water-to-light map:" => almanac.light = map,
                "light-to-temperature map:" => almanac.temperature = map,
                "temperature-to-humidity map:" => almanac.humidity = map,
                "humidity-to-location map:" => almanac.location = map,
                _ => (),
            }
        }
        almanac
    }

    fn find_seed_location(&self, seed: u64) -> u64 {
        let soil = scan_ranges(&self.soil, seed);
        let fertilizer = scan_ranges(&self.fertilizer, soil);
        let water = scan_ranges(&self.water, fertilizer);
        let light = scan_ranges(&self.light, water);
        let temperature = scan_ranges(&self.temperature, light);
        let humidity = scan_ranges(&self.humidity, temperature);
        scan_ranges(&self.location, humidity)
    }

    fn find_lowest_location(&self) -> u64 {
        let mut lowest_location = self.find_seed_location(self.seeds[0]);
        for seed in &self.seeds[1..] {
            if self.find_seed_location(*seed) < lowest_location {
                lowest_location = self.find_seed_location(*seed);
            }
        }
        lowest_location
    }
}

fn scan_ranges(ranges: &Vec<[u64; 3]>, value: u64) -> u64 {
    let mut result = value;
    for range in ranges {
        if value <= range[1] + range[2] - 1 && value >= range[1] {
            result = range[0] + value - range[1];
            break;
        }
    }
    result
}

fn main() {
    let filename = "src/bin/day05/input.txt";
    println!("Part 1: {}", part_1(filename));
}

fn part_1(filename: &str) -> u64 {
    let almanac = Almanac::new(filename);
    almanac.find_lowest_location()
}

fn parse_maps(filename: &str) -> (Vec<u64>, Vec<(String, Vec<[u64; 3]>)>) {
    let lines = advent_of_code_2023::read_lines(filename).unwrap();
    let seeds_str = lines[0].split(":").nth(1).unwrap().trim();
    let seeds = seeds_str
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let map_lines = lines[2..].to_vec();
    let mut current: Option<String> = None;
    let mut maps: Vec<(String, Vec<[u64; 3]>)> = Vec::new();
    let mut map: Vec<[u64; 3]> = Vec::new();

    for line in map_lines {
        if line.trim().is_empty() {
            if let Some(map_name) = current.take() {
                maps.push((map_name, map.clone()));
                map.clear();
            }
        } else if line.contains("-to-") {
            current = Some(line.clone());
        } else {
            let row = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
                .try_into()
                .unwrap();
            map.push(row);
        }
    }

    if let Some(map_name) = current {
        maps.push((map_name, map));
    }

    (seeds, maps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_maps() {
        let (seeds, maps) = parse_maps("src/bin/day05/test_input.txt");
        assert_eq!(seeds, vec![79, 14, 55, 13]);
        assert_eq!(maps.len(), 7);
        assert_eq!(maps[0].0, "seed-to-soil map:");
        assert_eq!(maps[0].1.len(), 2);
        assert_eq!(maps[1].0, "soil-to-fertilizer map:");
        assert_eq!(maps[1].1.len(), 3);
        assert_eq!(maps[2].0, "fertilizer-to-water map:");
        assert_eq!(maps[2].1.len(), 4);
        assert_eq!(maps[3].0, "water-to-light map:");
        assert_eq!(maps[3].1.len(), 2);
        assert_eq!(maps[4].0, "light-to-temperature map:");
        assert_eq!(maps[4].1.len(), 3);
        assert_eq!(maps[5].0, "temperature-to-humidity map:");
        assert_eq!(maps[5].1.len(), 2);
        assert_eq!(maps[6].0, "humidity-to-location map:");
        assert_eq!(maps[6].1.len(), 2);
    }

    #[test]
    fn test_almanac_new() {
        let almanac = Almanac::new("src/bin/day05/test_input.txt");
        assert_eq!(almanac.seeds, vec![79, 14, 55, 13]);
        assert_eq!(almanac.soil.len(), 2);
        assert_eq!(almanac.fertilizer.len(), 3);
        assert_eq!(almanac.water.len(), 4);
        assert_eq!(almanac.light.len(), 2);
        assert_eq!(almanac.temperature.len(), 3);
        assert_eq!(almanac.humidity.len(), 2);
        assert_eq!(almanac.location.len(), 2);
    }

    #[test]
    fn test_scan_ranges() {
        let ranges = vec![[1, 10, 10], [11, 20, 10], [21, 30, 10]];
        assert_eq!(scan_ranges(&ranges, 0), 0);
        assert_eq!(scan_ranges(&ranges, 1), 1);
        assert_eq!(scan_ranges(&ranges, 10), 1);
        assert_eq!(scan_ranges(&ranges, 11), 2);
        assert_eq!(scan_ranges(&ranges, 20), 11);
        assert_eq!(scan_ranges(&ranges, 21), 12);
        assert_eq!(scan_ranges(&ranges, 30), 21);
        assert_eq!(scan_ranges(&ranges, 41), 41);
    }

    #[test]
    fn test_find_seed_location() {
        let almanac = Almanac::new("src/bin/day05/test_input.txt");
        assert_eq!(almanac.find_seed_location(79), 82);
        assert_eq!(almanac.find_seed_location(14), 43);
        assert_eq!(almanac.find_seed_location(55), 86);
        assert_eq!(almanac.find_seed_location(13), 35);
    }

    #[test]
    fn test_find_lowest_location() {
        let almanac = Almanac::new("src/bin/day05/test_input.txt");
        assert_eq!(almanac.find_lowest_location(), 35);
    }
}
