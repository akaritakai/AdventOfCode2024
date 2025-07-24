use crate::puzzle::Puzzle;
use lazy_regex::regex_captures;
use z3::ast::{Ast, BV};
use z3::{Context, Optimize};

pub struct Day {
    a: i32,
    b: i32,
    c: i32,
    program: Vec<i32>,
}

impl Puzzle for Day {
    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_1(&self) -> String {
        let mut vm = ClassicVm::new(self.a, self.b, self.c, self.program.clone());
        let output = vm.run();
        output
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_2(&self) -> String {
        let ctx = Context::new(&z3::Config::default());
        let mut vm = SymbolicVm::new(&ctx, self.b, self.c, self.program.clone());
        vm.run()
    }
}

struct ClassicVm {
    ip: i32,
    a: i32,
    b: i32,
    c: i32,
    program: Vec<i32>,
}

impl ClassicVm {
    fn new(a: i32, b: i32, c: i32, program: Vec<i32>) -> Self {
        ClassicVm {
            ip: 0,
            a,
            b,
            c,
            program,
        }
    }

    fn combo(&self, x: i32) -> i32 {
        match x {
            0..=3 => x,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }

    fn run(&mut self) -> Vec<i32> {
        let mut output = Vec::new();
        while self.ip >= 0 && self.ip < self.program.len() as i32 {
            let opcode = self.program[self.ip as usize];
            let operand = self.program[self.ip as usize + 1];
            match opcode {
                /*adv*/ 0 => self.a >>= self.combo(operand),
                /*bxl*/ 1 => self.b ^= operand,
                /*bst*/ 2 => self.b = self.combo(operand) % 8,
                /*jnz*/
                3 => {
                    if self.a != 0 {
                        self.ip = operand - 2;
                    }
                }
                /*bxc*/ 4 => self.b ^= self.c,
                /*out*/ 5 => output.push(self.combo(operand) % 8),
                /*bdv*/ 6 => self.b = self.a >> self.combo(operand),
                /*cdv*/ 7 => self.c = self.a >> self.combo(operand),
                _ => unreachable!(),
            }
            self.ip += 2;
        }
        output
    }
}

struct SymbolicVm<'ctx> {
    context: &'ctx Context,
    ip: i32,
    a: BV<'ctx>,
    b: BV<'ctx>,
    c: BV<'ctx>,
    program: Vec<i32>,
    solver: Optimize<'ctx>,
}

impl<'ctx> SymbolicVm<'ctx> {
    fn new(ctx: &'ctx Context, b: i32, c: i32, program: Vec<i32>) -> Self {
        SymbolicVm {
            context: ctx,
            ip: 0,
            a: BV::new_const(ctx, "a", 128),
            b: BV::from_i64(ctx, b as i64, 128),
            c: BV::from_i64(ctx, c as i64, 128),
            program,
            solver: Optimize::new(ctx),
        }
    }

    fn literal(&self, operand: i32) -> BV<'ctx> {
        BV::from_i64(self.context, operand as i64, 128)
    }

    fn combo(&self, operand: i32) -> BV<'ctx> {
        match operand {
            0..=3 => self.literal(operand),
            4 => self.a.clone(),
            5 => self.b.clone(),
            6 => self.c.clone(),
            _ => unreachable!(),
        }
    }

    fn run(&mut self) -> String {
        let start_a = self.a.clone();
        let mut i = 0;
        loop {
            if self.ip < 0 || self.ip >= self.program.len() as i32 {
                break;
            }
            let opcode = self.program[self.ip as usize];
            let operand = self.program[self.ip as usize + 1];
            match opcode {
                /*adv*/ 0 => self.a = self.a.bvlshr(&self.combo(operand)),
                /*bxl*/ 1 => self.b = self.b.bvxor(&self.literal(operand)),
                /*bst*/ 2 => self.b = self.combo(operand).bvand(&self.literal(7)),
                /*jnz*/
                3 => {
                    if i == self.program.len() {
                        self.solver.assert(&self.a._eq(&self.literal(0)));
                        break;
                    } else {
                        self.ip = operand - 2;
                    }
                }
                /*bxc*/ 4 => self.b = self.b.bvxor(&self.c),
                /*out*/
                5 => {
                    if i < self.program.len() {
                        let lhs = self.combo(operand).bvand(&self.literal(7));
                        let rhs = self.literal(self.program[i]);
                        self.solver.assert(&lhs._eq(&rhs));
                        i += 1;
                    }
                }
                /*bdv*/ 6 => self.b = self.a.bvlshr(&self.combo(operand)),
                /*cdv*/ 7 => self.c = self.a.bvlshr(&self.combo(operand)),
                _ => unreachable!(),
            }
            self.ip += 2;
        }
        self.solver.minimize(&start_a);
        self.solver.check(&[]);
        let model = self.solver.get_model().unwrap();
        model
            .eval(&start_a, false)
            .unwrap()
            .as_u64()
            .unwrap()
            .to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let (_, a) = regex_captures!(r#"Register A: (\d+)"#, input).unwrap();
        let (_, b) = regex_captures!(r#"Register B: (\d+)"#, input).unwrap();
        let (_, c) = regex_captures!(r#"Register C: (\d+)"#, input).unwrap();
        let (_, program) = regex_captures!(r#"Program: (.*)"#, input).unwrap();
        let program = program
            .split(",")
            .map(|s| s.trim().parse().unwrap())
            .collect::<Vec<i32>>();
        Box::new(Day {
            a: a.parse().unwrap(),
            b: b.parse().unwrap(),
            c: c.parse().unwrap(),
            program,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "\
            Register A: 729\n\
            Register B: 0\n\
            Register C: 0\n\
            \n\
            Program: 0,1,5,4,3,0";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/17")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "3,5,0,1,5,1,5,1,0");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            Register A: 2024\n\
            Register B: 0\n\
            Register C: 0\n\
            \n\
            Program: 0,3,5,4,3,0";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "117440");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/17")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "107413700225434");
    }
}
