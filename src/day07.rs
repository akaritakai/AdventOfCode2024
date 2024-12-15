use crate::puzzle::Puzzle;

pub struct Day {
    equations: Vec<Equation>,
}

impl Puzzle for Day {
    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_1(&self) -> String {
        let mut sum = 0;
        for equation in &self.equations {
            if test_equation(
                equation.operands[0],
                equation.test_value,
                &equation.operands[1..],
                false,
            ) {
                sum += equation.test_value;
            }
        }
        sum.to_string()
    }

    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_2(&self) -> String {
        let mut sum = 0;
        for equation in &self.equations {
            if test_equation(
                equation.operands[0],
                equation.test_value,
                &equation.operands[1..],
                true,
            ) {
                sum += equation.test_value;
            }
        }
        sum.to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let equations = input
            .lines()
            .map(|line| {
                let mut parts = line.split(": ");
                let test_value = parts.next().unwrap().parse().unwrap();
                let operands = parts
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|operand| operand.parse().unwrap())
                    .collect();
                Equation {
                    test_value,
                    operands,
                }
            })
            .collect();
        Box::new(Day { equations })
    }
}

struct Equation {
    test_value: u64,
    operands: Vec<u64>,
}

fn test_equation(current: u64, expected: u64, remainder: &[u64], part_2: bool) -> bool {
    if remainder.is_empty() {
        return current == expected;
    }
    if current > expected {
        return false;
    }
    test_equation(current + remainder[0], expected, &remainder[1..], part_2)
        || test_equation(current * remainder[0], expected, &remainder[1..], part_2)
        || (part_2
            && test_equation(
                format!("{}{}", current, remainder[0]).parse().unwrap(),
                expected,
                &remainder[1..],
                part_2,
            ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "\
            190: 10 19\n\
            3267: 81 40 27\n\
            83: 17 5\n\
            156: 15 6\n\
            7290: 6 8 6 15\n\
            161011: 16 10 13\n\
            192: 17 8 14\n\
            21037: 9 7 18 13\n\
            292: 11 6 16 20";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "3749");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/07")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "20665830408335");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            190: 10 19\n\
            3267: 81 40 27\n\
            83: 17 5\n\
            156: 15 6\n\
            7290: 6 8 6 15\n\
            161011: 16 10 13\n\
            192: 17 8 14\n\
            21037: 9 7 18 13\n\
            292: 11 6 16 20";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "11387");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/07")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "354060705047464");
    }
}
