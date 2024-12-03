use crate::puzzle::Puzzle;
use lazy_regex::regex;

pub struct Day {
    memory: String,
}

impl Puzzle for Day {
    fn solve_part_1(&self) -> String {
        let re = regex!(r"mul\((\d+),(\d+)\)");
        re.captures_iter(&self.memory)
            .map(|cap| cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap())
            .sum::<i32>()
            .to_string()
    }

    fn solve_part_2(&self) -> String {
        let re = regex!(r"(?:do\(\))|(?:don't\(\))|mul\((\d+),(\d+)\)");
        let mut sum = 0;
        let mut enabled = true;
        for cap in re.captures_iter(&self.memory) {
            if &cap[0] == "do()" {
                enabled = true;
            } else if &cap[0] == "don't()" {
                enabled = false;
            } else if enabled {
                sum += cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap();
            }
        }
        sum.to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        Box::new(Day {
            memory: input.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "161");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/03")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "165225049");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "48");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/03")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "108830766");
    }
}
