// --- Day 9: Mirage Maintenance ---
struct History(Vec<Vec<i64>>);

impl History {
    fn new(line: &str) -> Self {
        let first: Vec<i64> = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        Self(vec![first])
    }

    fn add(&mut self, v: Vec<i64>) {
        self.0.push(v);
    }

    fn last_of_nth(&self, n: usize) -> Option<&i64> {
        self.0.get(n).and_then(|x| x.last())
    }

    fn first_of_nth(&self, n: usize) -> Option<&i64> {
        self.0.get(n).and_then(|x| x.first())
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn add_to_array(&mut self, idx: usize, value: i64) {
        self.0[idx].push(value);
    }

    fn insert_into_array(&mut self, idx: usize, value: i64) {
        self.0[idx].insert(0, value);
    }

    fn generate_sequence(&mut self) {
        let mut current = 0;
        let mut next = diff_array(self.0[current].as_slice());
        while !all_zeros(next.as_slice()) {
            self.add(next);
            current += 1;
            next = diff_array(self.0[current].as_slice());
        }
        self.add(next);
    }

    fn extrapolate(line: String) -> i64 {
        let mut h = History::new(&line);
        h.generate_sequence();
        h.add_to_array(h.len() - 1, 0);
        for i in (0..h.len() - 1).rev() {
            let value = *h.last_of_nth(i).unwrap() + *h.last_of_nth(i + 1).unwrap();
            h.add_to_array(i, value);
        }
        *h.last_of_nth(0).unwrap()
    }

    fn extrapolate_backwards(line: String) -> i64 {
        let mut h = History::new(&line);
        h.generate_sequence();
        h.insert_into_array(h.len() - 1, 0);
        for i in (0..h.len() - 1).rev() {
            let value = *h.first_of_nth(i).unwrap() - *h.first_of_nth(i + 1).unwrap();
            h.insert_into_array(i, value);
        }
        *h.first_of_nth(0).unwrap()
    }
}

fn all_zeros(arr: &[i64]) -> bool {
    arr.iter().all(|&x| x == 0)
}

fn diff_array(arr: &[i64]) -> Vec<i64> {
    let mut result = Vec::new();
    for i in 0..arr.len() - 1 {
        result.push(arr[i + 1] - arr[i]);
    }
    result
}

fn history_processor<F1>(filename: &str, extrapolate: F1) -> i64
    where
        F1: Fn(String) -> i64,
{
    advent_of_code_2023::read_lines(filename).unwrap()
        .into_iter()
        .map(extrapolate)
        .sum()
}

fn part_1(filename: &str) -> i64 {
    history_processor(filename, History::extrapolate)
}

fn part_2(filename: &str) -> i64 {
    history_processor(filename, History::extrapolate_backwards)
}

fn main() {
    println!("part 1: {}", part_1("src/bin/day09/input.txt"));
    println!("part 2: {}", part_2("src/bin/day09/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_all_zeros() {
        assert_eq!(all_zeros(&[0, 0, 0]), true);
        assert_eq!(all_zeros(&[0, 0, 1]), false);
        assert_eq!(all_zeros(&[0, 1, 0]), false);
        assert_eq!(all_zeros(&[1, 0, 0]), false);
        assert_eq!(all_zeros(&[1, 1, 1]), false);
    }

    #[test]
    fn test_diff_array() {
        assert_eq!(diff_array(&[1, 2, 3]), vec![1, 1]);
        assert_eq!(diff_array(&[1, 2, 4]), vec![1, 2]);
        assert_eq!(diff_array(&[1, 3, 6]), vec![2, 3]);
        assert_eq!(diff_array(&[1, 4, 10]), vec![3, 6]);
    }

    #[test]
    fn test_history() {
        let mut history = History::new("1 2 3");
        assert_eq!(history.len(), 1);
        assert_eq!(history.0.last().unwrap(), &vec![1, 2, 3]);
        history.add(vec![1, 2, 4]);
        assert_eq!(history.len(), 2);
        assert_eq!(history.0.last().unwrap(), &vec![1, 2, 4]);
        history.add_to_array(0, 4);
        assert_eq!(history.len(), 2);
        assert_eq!(history.0.last().unwrap(), &vec![1, 2, 4]);
    }

    #[test]
    fn test_generate_sequences() {
        let mut history = History::new("0 3 6 9 12 15");
        history.generate_sequence();
        assert_eq!(history.len(), 3);
        assert_eq!(all_zeros(history.0.last().unwrap()), true);

        let mut history = History::new("1 3 6 10 15 21");
        history.generate_sequence();
        assert_eq!(history.len(), 4);
        assert_eq!(all_zeros(history.0.last().unwrap()), true);

        let mut history = History::new("10 13 16 21 30 45");
        history.generate_sequence();
        assert_eq!(history.len(), 5);
        assert_eq!(all_zeros(history.0.last().unwrap()), true);
    }

    #[test]
    fn test_extrapolate() {
        let res = History::extrapolate("0 3 6 9 12 15".to_string());
        assert_eq!(res, 18);

        let res = History::extrapolate("1 3 6 10 15 21".to_string());
        assert_eq!(res, 28);

        let res = History::extrapolate("10 13 16 21 30 45".to_string());
        assert_eq!(res, 68);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/bin/day09/test_input.txt"), 114);
    }

    #[test]
    fn test_extrapolate_backwards() {
        let res = History::extrapolate_backwards("0 3 6 9 12 15".to_string());
        assert_eq!(res, -3);

        let res = History::extrapolate_backwards("1 3 6 10 15 21".to_string());
        assert_eq!(res, 0);

        let res = History::extrapolate_backwards("10 13 16 21 30 45".to_string());
        assert_eq!(res, 5);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("src/bin/day09/test_input.txt"), 2);
    }
}
