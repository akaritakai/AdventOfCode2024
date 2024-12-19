use crate::puzzle::Puzzle;
use lazy_regex::regex;
use rayon::prelude::*;
use std::ops::{Add, Mul};
use z3::ast::{Ast, Int};
use z3::{Context, Optimize};

pub struct Day {
    machines: Vec<ClawMachine>,
}

impl Puzzle for Day {
    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_1(&self) -> String {
        self.machines
            .par_iter()
            .filter_map(|machine| {
                (0..100)
                    .filter_map(|a| {
                        let location = (machine.a.0 * a, machine.a.1 * a);
                        if location.0 > machine.prize.0 || location.1 > machine.prize.1 {
                            return None;
                        }
                        let b = ((machine.prize.0 - location.0) / machine.b.0).max(0);
                        if location.0 + machine.b.0 * b == machine.prize.0
                            && location.1 + machine.b.1 * b == machine.prize.1
                        {
                            Some(3 * a + b)
                        } else {
                            None
                        }
                    })
                    .min()
            })
            .sum::<usize>()
            .to_string()
    }

    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_2(&self) -> String {
        let ctx = &Context::new(&z3::Config::default());
        self.machines
            .iter()
            .map(|machine| ClawMachine {
                a: machine.a,
                b: machine.b,
                prize: (
                    machine.prize.0 + 10000000000000,
                    machine.prize.1 + 10000000000000,
                ),
            })
            .filter_map(|machine| {
                let opt = Optimize::new(ctx);
                let a = Int::new_const(ctx, "a");
                let b = Int::new_const(ctx, "b");
                let a_x = Int::from_u64(ctx, machine.a.0 as u64);
                let a_y = Int::from_u64(ctx, machine.a.1 as u64);
                let b_x = Int::from_u64(ctx, machine.b.0 as u64);
                let b_y = Int::from_u64(ctx, machine.b.1 as u64);
                let p_x = Int::from_u64(ctx, machine.prize.0 as u64);
                let p_y = Int::from_u64(ctx, machine.prize.1 as u64);
                let cost = a.clone().mul(&Int::from_u64(ctx, 3)).add(&b.clone());
                let assumptions = [
                    a.clone().ge(&Int::from_u64(ctx, 0)),
                    b.clone().ge(&Int::from_u64(ctx, 0)),
                    a.clone().mul(&a_x).add(&b.clone().mul(&b_x))._eq(&p_x),
                    a.clone().mul(&a_y).add(&b.clone().mul(&b_y))._eq(&p_y),
                ];
                opt.minimize(&cost);
                if opt.check(&assumptions) == z3::SatResult::Sat {
                    let model = opt.get_model().unwrap();
                    let cost = model.eval(&cost, false).unwrap().as_u64().unwrap() as usize;
                    Some(cost)
                } else {
                    None
                }
            })
            .sum::<usize>()
            .to_string()
    }
}

#[derive(Debug)]
struct ClawMachine {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let machines = input
            .split("\n\n")
            .map(|block| {
                let mut lines = block.lines();
                ClawMachine {
                    a: parse_digits(lines.next().unwrap()),
                    b: parse_digits(lines.next().unwrap()),
                    prize: parse_digits(lines.next().unwrap()),
                }
            })
            .collect();
        Box::new(Day { machines })
    }
}

fn parse_digits(line: &str) -> (usize, usize) {
    let re = regex!(r"\D+(\d+)\D+(\d+)");
    let cap = re.captures(line).unwrap();
    (cap[1].parse().unwrap(), cap[2].parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "\
            Button A: X+94, Y+34\n\
            Button B: X+22, Y+67\n\
            Prize: X=8400, Y=5400\n\
            \n\
            Button A: X+26, Y+66\n\
            Button B: X+67, Y+21\n\
            Prize: X=12748, Y=12176\n\
            \n\
            Button A: X+17, Y+86\n\
            Button B: X+84, Y+37\n\
            Prize: X=7870, Y=6450\n\
            \n\
            Button A: X+69, Y+23\n\
            Button B: X+27, Y+71\n\
            Prize: X=18641, Y=10279";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "480");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/13")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "35997");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/13")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "82510994362072");
    }
}
