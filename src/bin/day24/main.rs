fn main() {
    let time_start = std::time::Instant::now();
    let count = part_1("src/bin/day24/input.txt");
    println!(
        "Part 1: {:?}  Time: {}μs",
        count,
        time_start.elapsed().as_micros()
    );
}

fn part_1(filename: &str) -> usize {
    let input = advent_of_code_2023::read_lines(filename).unwrap();
    let hailstones = make_hailstones(input);
    let intersections = find_intersections(
        &hailstones,
        Bounds {
            x_min: 200000000000000.0,
            x_max: 400000000000000.0,
            y_min: 200000000000000.0,
            y_max: 400000000000000.0,
        },
    );
    intersections.len()
}

struct Bounds {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
}

impl Bounds {
    fn within(&self, x: f64, y: f64) -> bool {
        x >= self.x_min && x <= self.x_max && y >= self.y_min && y <= self.y_max
    }
}

#[derive(Debug, PartialEq)]
struct Hailstone {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    a: f64,
    b: f64,
    c: f64,
}

impl Hailstone {
    fn new(input: &str) -> Hailstone {
        // 19, 13, 30 @ -2,  1, -2
        let mut parts = input.split(" @ ");
        let pos_str = parts.next().unwrap();
        let mut pos_parts = pos_str.split(", ");
        let x = pos_parts.next().unwrap().trim().parse::<f64>().unwrap();
        let y = pos_parts.next().unwrap().trim().parse::<f64>().unwrap();
        let z = pos_parts.next().unwrap().trim().parse::<f64>().unwrap();
        let vel_str = parts.next().unwrap();
        let mut vel_parts = vel_str.split(", ");
        let vx = vel_parts.next().unwrap().trim().parse::<f64>().unwrap();
        let vy = vel_parts.next().unwrap().trim().parse::<f64>().unwrap();
        let vz = vel_parts.next().unwrap().trim().parse::<f64>().unwrap();
        let a = vy;
        let b = -vx;
        let c = vy * x - vx * y;
        Hailstone {
            x,
            y,
            z,
            vx,
            vy,
            vz,
            a,
            b,
            c,
        }
    }

    fn parallel(&self, other: &Hailstone) -> bool {
        self.a * other.b == self.b * other.a
    }
}

fn make_hailstones(input: Vec<String>) -> Vec<Hailstone> {
    input.iter().map(|s| Hailstone::new(s)).collect()
}

fn find_intersections(hailstones: &Vec<Hailstone>, bounds: Bounds) -> Vec<(usize, usize)> {
    let mut intersections = Vec::new();
    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            let h1 = &hailstones[i];
            let h2 = &hailstones[j];
            if h1.parallel(h2) {
                continue;
            }
            let x = (h2.b * h1.c - h1.b * h2.c) / (h1.a * h2.b - h2.a * h1.b);
            let y = (h1.a * h2.c - h2.a * h1.c) / (h1.a * h2.b - h2.a * h1.b);
            if !bounds.within(x, y) {
                continue;
            }
            if vec![h1, h2]
                .iter()
                .all(|h| (x - h.x) * h.vx >= 0.0 && (y - h.y) * h.vy >= 0.0)
            {
                intersections.push((i, j));
            }
        }
    }
    intersections
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2023::read_lines;

    #[test]
    fn test_hailstones() {
        let input = vec![
            "19, 13, 30 @ -2,  1, -2".to_string(),
            "18, 19, 22 @ -1, -1, -2".to_string(),
        ];
        let mut hailstones = make_hailstones(input);
        assert_eq!(hailstones.len(), 2);
        assert_eq!(
            hailstones[0],
            Hailstone {
                x: 19.0,
                y: 13.0,
                z: 30.0,
                vx: -2.0,
                vy: 1.0,
                vz: -2.0,
                a: 1.0,
                b: 2.0,
                c: 45.0,
            }
        );
        assert_eq!(
            hailstones[1],
            Hailstone {
                x: 18.0,
                y: 19.0,
                z: 22.0,
                vx: -1.0,
                vy: -1.0,
                vz: -2.0,
                a: -1.0,
                b: 1.0,
                c: 1.0,
            }
        );
    }

    #[test]
    fn test_find_intersections() {
        let input = read_lines("src/bin/day24/test_input.txt").unwrap();
        let hailstones = make_hailstones(input);
        let intersections = find_intersections(
            &hailstones,
            Bounds {
                x_min: 7.0,
                x_max: 27.0,
                y_min: 7.0,
                y_max: 27.0,
            },
        );
        assert_eq!(intersections.len(), 2);
    }
}
