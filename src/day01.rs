use crate::puzzle::Puzzle;
use std::collections::HashMap;

pub struct Day {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl Puzzle for Day {
    fn solve_part_1(&self) -> String {
        let mut left = self.left.clone();
        let mut right = self.right.clone();
        left.sort_unstable();
        right.sort_unstable();
        left.iter()
            .zip(right.iter())
            .map(|(l, r)| (l - r).abs())
            .sum::<i32>()
            .to_string()
    }

    fn solve_part_2(&self) -> String {
        let mut freq = HashMap::new();
        for r in self.right.iter() {
            *freq.entry(r).or_insert(0) += 1;
        }
        self.left
            .iter()
            .filter_map(|&i| freq.get(&i).map(|&c| i * c))
            .sum::<i32>()
            .to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let mut left = Vec::new();
        let mut right = Vec::new();
        for line in input.lines() {
            let mut parts = line.split_whitespace();
            left.push(parts.next().unwrap().parse().unwrap());
            right.push(parts.next().unwrap().parse().unwrap());
        }
        Box::new(Day { left, right })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "\
            3 4\n\
            4 3\n\
            2 5\n\
            1 3\n\
            3 9\n\
            3 3";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "11");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/01")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "1666427");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            3 4\n\
            4 3\n\
            2 5\n\
            1 3\n\
            3 9\n\
            3 3";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "31");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/01")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "24316233");
    }
}
