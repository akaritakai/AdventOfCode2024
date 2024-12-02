use crate::puzzle::Puzzle;

pub struct Day {
    levels: Vec<Vec<i32>>,
}

impl Puzzle for Day {
    fn solve_part_1(&self) -> String {
        self.levels
            .iter()
            .filter(|level| is_safe(level))
            .count()
            .to_string()
    }

    fn solve_part_2(&self) -> String {
        self.levels
            .iter()
            .filter(|level| is_safe_with_removal(level))
            .count()
            .to_string()
    }
}

fn is_safe_with_removal(level: &[i32]) -> bool {
    if is_safe(level) {
        return true;
    }
    for i in 0..level.len() {
        let mut modified = level.to_vec();
        modified.remove(i);
        if is_safe(&modified) {
            return true;
        }
    }
    false
}

fn is_safe(level: &[i32]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;
    for window in level.windows(2) {
        let diff = window[1] - window[0];
        if !(1..=3).contains(&diff) {
            increasing = false;
        }
        if !(-3..=-1).contains(&diff) {
            decreasing = false;
        }
        if !increasing && !decreasing {
            return false;
        }
    }
    increasing || decreasing
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let mut levels = Vec::new();
        for line in input.lines() {
            let mut level = Vec::new();
            for part in line.split_whitespace() {
                level.push(part.parse().unwrap());
            }
            levels.push(level);
        }
        Box::new(Day { levels })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "\
            7 6 4 2 1\n\
            1 2 7 8 9\n\
            9 7 6 2 1\n\
            1 3 2 4 5\n\
            8 6 4 4 1\n\
            1 3 6 7 9";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "2");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/02")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "585");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            7 6 4 2 1\n\
            1 2 7 8 9\n\
            9 7 6 2 1\n\
            1 3 2 4 5\n\
            8 6 4 4 1\n\
            1 3 6 7 9";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "4");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/02")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "626");
    }
}
