// --- Day 5: If You Give A Seed A Fertilizer ---
#[derive(Default, Debug)]
struct Almanac {
    fertilizer: Vec<[u64; 3]>,
    fertilizer_ranges: Vec<(u64, (u64, u64))>,
    humidity: Vec<[u64; 3]>,
    humidity_ranges: Vec<(u64, (u64, u64))>,
    light: Vec<[u64; 3]>,
    light_ranges: Vec<(u64, (u64, u64))>,
    location: Vec<[u64; 3]>,
    location_ranges: Vec<(u64, (u64, u64))>,
    seed_ranges: Vec<[u64; 2]>,
    seeds: Vec<u64>,
    soil: Vec<[u64; 3]>,
    soil_ranges: Vec<(u64, (u64, u64))>,
    temperature: Vec<[u64; 3]>,
    temperature_ranges: Vec<(u64, (u64, u64))>,
    water: Vec<[u64; 3]>,
    water_ranges: Vec<(u64, (u64, u64))>,
}

impl Almanac {
    fn new(filename: &str) -> Almanac {
        let (seeds, maps) = parse_maps(filename);
        let mut almanac = Almanac::default();
        almanac.seeds = seeds;
        almanac.find_seed_ranges();
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
        almanac.soil_ranges = maps_to_ranges(&almanac.soil);
        almanac.fertilizer_ranges = maps_to_ranges(&almanac.fertilizer);
        almanac.water_ranges = maps_to_ranges(&almanac.water);
        almanac.light_ranges = maps_to_ranges(&almanac.light);
        almanac.temperature_ranges = maps_to_ranges(&almanac.temperature);
        almanac.humidity_ranges = maps_to_ranges(&almanac.humidity);
        almanac.location_ranges = maps_to_ranges(&almanac.location);
        almanac
    }

    fn find_single_seed_location(&self, seed: u64) -> u64 {
        let soil = scan_ranges_by_seed(&self.soil, seed);
        let fertilizer = scan_ranges_by_seed(&self.fertilizer, soil);
        let water = scan_ranges_by_seed(&self.water, fertilizer);
        let light = scan_ranges_by_seed(&self.light, water);
        let temperature = scan_ranges_by_seed(&self.temperature, light);
        let humidity = scan_ranges_by_seed(&self.humidity, temperature);
        scan_ranges_by_seed(&self.location, humidity)
    }

    fn find_lowest_location_by_seed(&self) -> u64 {
        let mut lowest_location = self.find_single_seed_location(self.seeds[0]);
        for seed in &self.seeds[1..] {
            if self.find_single_seed_location(*seed) < lowest_location {
                lowest_location = self.find_single_seed_location(*seed);
            }
        }
        lowest_location
    }

    fn find_seed_ranges(&mut self) {
        let mut i = 0;
        while i < &self.seeds.len() - 1 {
            self.seed_ranges
                .push([self.seeds[i], self.seeds[i] + self.seeds[i + 1] - 1]);
            i += 2;
        }
        self.seed_ranges.sort_by(|a, b| a[0].cmp(&b[0]));
    }

    fn find_lowest_location_by_ranges(&self) -> u64 {
        let mapped_soil = map_all_ranges(&self.seed_ranges, &self.soil_ranges);
        let mapped_fertilizer = map_all_ranges(&mapped_soil, &self.fertilizer_ranges);
        let mapped_water = map_all_ranges(&mapped_fertilizer, &self.water_ranges);
        let mapped_light = map_all_ranges(&mapped_water, &self.light_ranges);
        let mapped_temperature = map_all_ranges(&mapped_light, &self.temperature_ranges);
        let mapped_humidity = map_all_ranges(&mapped_temperature, &self.humidity_ranges);
        let mapped_location = map_all_ranges(&mapped_humidity, &self.location_ranges);
        mapped_location[0][0]
    }
}

fn maps_to_ranges(maps: &Vec<[u64; 3]>) -> Vec<(u64, (u64, u64))> {
    let mut ranges: Vec<(u64, (u64, u64))> = Vec::new();
    for map in maps {
        ranges.push((map[0], (map[1], map[1] + map[2] - 1)));
    }
    ranges.sort_by(|a, b| a.1.cmp(&b.1));
    ranges
}

fn scan_ranges_by_seed(ranges: &Vec<[u64; 3]>, value: u64) -> u64 {
    let mut result = value;
    for range in ranges {
        if value <= range[1] + range[2] - 1 && value >= range[1] {
            result = range[0] + value - range[1];
            break;
        }
    }
    result
}

fn scan_ranges_by_range(
    seed_range: [u64; 2],
    map_ranges: &Vec<(u64, (u64, u64))>,
) -> Vec<[u64; 2]> {
    let mut mapped: Vec<[u64; 2]> = Vec::new();
    let mut start = seed_range[0];
    let end = seed_range[1];

    if start > map_ranges[map_ranges.len() - 1].1 .1 || end < map_ranges[0].1 .0 {
        mapped.push([start, end]);
        return mapped;
    }

    for i in 0..map_ranges.len() {
        let min = map_ranges[i].1 .0;
        let max = map_ranges[i].1 .1;
        if start > max && i < map_ranges.len() - 1 {
            continue;
        } else if start > max && i == map_ranges.len() - 1 {
            mapped.push([start, end]);
            break;
        } else if end < min {
            mapped.push([start, end]);
            break;
        } else if start >= min && end <= max {
            mapped.push([map_ranges[i].0 + start - min, map_ranges[i].0 + end - min]);
            break;
        } else if start >= min && end > max {
            mapped.push([map_ranges[i].0 + start - min, map_ranges[i].0 + max - min]);
            start = max + 1;
            continue;
        } else if start < min && end <= max {
            mapped.push([start, min - 1]);
            mapped.push([map_ranges[i].0, map_ranges[i].0 + end - min]);
            break;
        } else if start < min && end > max {
            mapped.push([start, min - 1]);
            mapped.push([map_ranges[i].0, map_ranges[i].0 + max - min]);
            start = max + 1;
            continue;
        }
    }

    if end > map_ranges[map_ranges.len() - 1].1 .1 {
        mapped.push([start, end]);
    }

    if mapped.is_empty() {
        mapped = vec![[start, end]];
    } else {
        mapped.sort_by(|a, b| a[0].cmp(&b[0]));
    }

    mapped
}

fn map_all_ranges(
    seed_ranges: &Vec<[u64; 2]>,
    map_ranges: &Vec<(u64, (u64, u64))>,
) -> Vec<[u64; 2]> {
    let mut mapped: Vec<[u64; 2]> = Vec::new();
    for seed_range in seed_ranges {
        let mut mapped_ranges = scan_ranges_by_range(*seed_range, map_ranges);
        mapped.append(&mut mapped_ranges);
    }
    mapped.sort_by(|a, b| a[0].cmp(&b[0]));
    mapped
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

fn main() {
    let filename = "src/bin/day05/input.txt";
    println!("Part 1: {}", part_1(filename));
    println!("Part 2: {:?}", part_2(filename));
}

fn part_1(filename: &str) -> u64 {
    let almanac = Almanac::new(filename);
    almanac.find_lowest_location_by_seed()
}

fn part_2(filename: &str) -> u64 {
    let almanac = Almanac::new(filename);
    almanac.find_lowest_location_by_ranges()
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
        assert_eq!(almanac.seed_ranges, vec![[55, 67], [79, 92]]);
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
        assert_eq!(scan_ranges_by_seed(&ranges, 0), 0);
        assert_eq!(scan_ranges_by_seed(&ranges, 1), 1);
        assert_eq!(scan_ranges_by_seed(&ranges, 10), 1);
        assert_eq!(scan_ranges_by_seed(&ranges, 11), 2);
        assert_eq!(scan_ranges_by_seed(&ranges, 20), 11);
        assert_eq!(scan_ranges_by_seed(&ranges, 21), 12);
        assert_eq!(scan_ranges_by_seed(&ranges, 30), 21);
        assert_eq!(scan_ranges_by_seed(&ranges, 41), 41);
    }

    #[test]
    fn test_find_seed_location() {
        let almanac = Almanac::new("src/bin/day05/test_input.txt");
        assert_eq!(almanac.find_single_seed_location(79), 82);
        assert_eq!(almanac.find_single_seed_location(14), 43);
        assert_eq!(almanac.find_single_seed_location(55), 86);
        assert_eq!(almanac.find_single_seed_location(13), 35);
    }

    #[test]
    fn test_find_lowest_location() {
        let almanac = Almanac::new("src/bin/day05/test_input.txt");
        assert_eq!(almanac.find_lowest_location_by_seed(), 35);
    }

    #[test]
    fn test_maps_to_ranges() {
        let maps = vec![[1, 10, 10], [11, 20, 10], [21, 30, 10]];
        let ranges = maps_to_ranges(&maps);
        assert_eq!(ranges.len(), 3);
        assert_eq!(ranges[0], (1, (10, 19)));
        assert_eq!(ranges[1], (11, (20, 29)));
        assert_eq!(ranges[2], (21, (30, 39)));
    }

    #[test]
    fn test_scan_ranges_by_range() {
        let seed_range = [55, 67];
        let map_ranges = vec![(101, (10, 19)), (11, (20, 59)), (1, (63, 66))];
        let scanned = scan_ranges_by_range(seed_range, &map_ranges);
        assert_eq!(scanned.len(), 4);
        assert_eq!(scanned, [[1, 4], [46, 50], [60, 62], [67, 67]]);
    }

    #[test]
    fn test_map_all_ranges() {
        let seed_ranges = vec![[55, 67], [79, 92]];
        let soil_map = vec![(52, (50, 97)), (50, (98, 99))];
        let soil_ranges = map_all_ranges(&seed_ranges, &soil_map);
        assert_eq!(soil_ranges.len(), 2);
        assert_eq!(soil_ranges, [[57, 69], [81, 94]]);
        let fertilizer_map = vec![(39, (0, 14)), (0, (15, 51)), (37, (52, 53))];
        let fertilizer_ranges = map_all_ranges(&soil_ranges, &fertilizer_map);
        assert_eq!(fertilizer_ranges.len(), 2);
        assert_eq!(fertilizer_ranges, [[57, 69], [81, 94]]);
        let water_map = vec![(42, (0, 6)), (57, (7, 10)), (0, (11, 52)), (49, (53, 60))];
        let water_ranges = map_all_ranges(&fertilizer_ranges, &water_map);
        assert_eq!(water_ranges.len(), 3);
        assert_eq!(water_ranges, [[53, 56], [61, 69], [81, 94]]);
        let light_map = vec![(88, (18, 24)), (18, (25, 94))];
        let light_ranges = map_all_ranges(&water_ranges, &light_map);
        assert_eq!(light_ranges.len(), 3);
        assert_eq!(light_ranges, [[46, 49], [54, 62], [74, 87]]);
        let temperature_map = vec![(81, (45, 63)), (68, (64, 76)), (45, (77, 99))];
        let temperature_ranges = map_all_ranges(&light_ranges, &temperature_map);
        assert_eq!(temperature_ranges.len(), 4);
        assert_eq!(temperature_ranges, [[45, 55], [78, 80], [82, 85], [90, 98]]);
        let humidity_map = vec![(1, (0, 68)), (0, (69, 69))];
        let humidity_ranges = map_all_ranges(&temperature_ranges, &humidity_map);
        assert_eq!(humidity_ranges.len(), 4);
        assert_eq!(humidity_ranges, [[46, 56], [78, 80], [82, 85], [90, 98]]);
        let location_map = vec![(60, (56, 92)), (56, (93, 96))];
        let location_ranges = map_all_ranges(&humidity_ranges, &location_map);
        assert_eq!(location_ranges.len(), 7);
        assert_eq!(
            location_ranges,
            [
                [46, 55],
                [56, 59],
                [60, 60],
                [82, 84],
                [86, 89],
                [94, 96],
                [97, 98]
            ]
        );
    }

    #[test]
    fn test_find_lowest_location_by_ranges() {
        let almanac = Almanac::new("src/bin/day05/test_input.txt");
        assert_eq!(almanac.find_lowest_location_by_ranges(), 46);
    }
}
