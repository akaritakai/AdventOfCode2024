use crate::puzzle::Puzzle;
use itertools::Itertools;
use lazy_regex::Regex;
use std::collections::HashMap;

pub struct Day {
    patterns: Vec<String>,
    designs: Vec<String>,
}

impl Puzzle for Day {
    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_1(&self) -> String {
        let regexp = format!("^({})*$", self.patterns.join("|"));
        let regexp = Regex::new(regexp.as_str()).unwrap();
        self.designs
            .iter()
            .filter(|design| regexp.is_match(design))
            .count()
            .to_string()
    }

    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_2(&self) -> String {
        let mut memo = HashMap::new();
        self.designs
            .iter()
            .map(|design| count_ways(design, &self.patterns, &mut memo))
            .sum::<usize>()
            .to_string()
    }
}

fn count_ways(design: &str, patterns: &Vec<String>, memo: &mut HashMap<String, usize>) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(&count) = memo.get(design) {
        return count;
    }
    let mut count = 0;
    for pattern in patterns {
        if design.starts_with(pattern) {
            let remaining = &design[pattern.len()..];
            count += count_ways(remaining, patterns, memo);
        }
    }
    memo.insert(design.to_string(), count);
    count
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let mut parts = input.split("\n\n");
        let patterns = parts
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.to_string())
            .collect();
        let designs = parts
            .next()
            .unwrap()
            .lines()
            .map(|s| s.to_string())
            .collect_vec();
        Box::new(Day { patterns, designs })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "\
            r, wr, b, g, bwu, rb, gb, br\n\
            \n\
            brwrr\n\
            bggr\n\
            gbbr\n\
            rrbgbr\n\
            ubwu\n\
            bwurrg\n\
            brgr\n\
            bbrgwb";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "6");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/19")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "367");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            r, wr, b, g, bwu, rb, gb, br\n\
            \n\
            brwrr\n\
            bggr\n\
            gbbr\n\
            rrbgbr\n\
            ubwu\n\
            bwurrg\n\
            brgr\n\
            bbrgwb";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "16");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/19")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "724388733465031");
    }
}
