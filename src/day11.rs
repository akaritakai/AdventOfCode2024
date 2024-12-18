use crate::puzzle::Puzzle;
use std::collections::HashMap;

pub struct Day {
    stones: Vec<usize>,
}

impl Puzzle for Day {
    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_1(&self) -> String {
        let mut dp = HashMap::new();
        self.stones
            .iter()
            .map(|&stone| calculate(stone, 25, &mut dp))
            .sum::<usize>()
            .to_string()
    }

    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_2(&self) -> String {
        let mut dp = HashMap::new();
        self.stones
            .iter()
            .map(|&stone| calculate(stone, 75, &mut dp))
            .sum::<usize>()
            .to_string()
    }
}

fn calculate(stone: usize, remaining: usize, dp: &mut HashMap<(usize, usize), usize>) -> usize {
    if remaining == 0 {
        return 1; // We have 1 stone in hand.
    }
    if let Some(&result) = dp.get(&(stone, remaining)) {
        return result;
    }
    if stone == 0 {
        let result = calculate(1, remaining - 1, dp);
        dp.insert((stone, remaining), result);
        return result;
    }
    let rep = stone.to_string();
    if rep.len() % 2 == 0 {
        let half = rep.len() / 2;
        let left = rep[..half].parse().unwrap();
        let right = rep[half..].parse().unwrap();
        let result = calculate(left, remaining - 1, dp) + calculate(right, remaining - 1, dp);
        dp.insert((stone, remaining), result);
        return result;
    }
    let result = calculate(stone * 2024, remaining - 1, dp);
    dp.insert((stone, remaining), result);
    result
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let stones = input
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        Box::new(Day { stones })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "125 17";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "55312");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/11")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "184927");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/11")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "220357186726677");
    }
}
