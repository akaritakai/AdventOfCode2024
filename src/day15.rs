use crate::puzzle::Puzzle;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Day {
    grid: Vec<Vec<char>>,
    moves: Vec<(i32, i32)>,
}

impl Puzzle for Day {
    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_1(&self) -> String {
        let mut warehouse = Warehouse::create(&self.grid, 1);
        for dir in &self.moves {
            warehouse.move_robot(*dir);
        }

        warehouse.gps_sum().to_string()
    }

    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_2(&self) -> String {
        let mut warehouse = Warehouse::create(&self.grid, 2);
        for dir in &self.moves {
            warehouse.move_robot(*dir);
        }
        warehouse.gps_sum().to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rectangle {
    row: i32,
    col: i32,
    width: i32,
}

impl Rectangle {
    fn create(row: i32, col: i32, width: i32) -> Self {
        Self { row, col, width }
    }

    fn translate(&self, dir: (i32, i32)) -> Self {
        Self {
            row: self.row + dir.0,
            col: self.col + dir.1,
            width: self.width,
        }
    }

    fn occupying_points(&self) -> Vec<(i32, i32)> {
        let mut points = Vec::new();
        for c in self.col..(self.col + self.width) {
            points.push((self.row, c));
        }
        points
    }
}

struct Warehouse {
    walls: HashSet<(i32, i32)>,
    boxes: HashMap<(i32, i32), Rectangle>,
    robot: (i32, i32),
}

impl Warehouse {
    fn create(grid: &[Vec<char>], width: i32) -> Self {
        let mut walls = HashSet::new();
        let mut boxes = HashMap::new();
        let mut robot = (0, 0);
        for (row, line) in grid.iter().enumerate() {
            for (col, ch) in line.iter().enumerate() {
                let col = width * col as i32;
                match ch {
                    '#' => {
                        let rect = Rectangle::create(row as i32, col, width);
                        for point in rect.occupying_points() {
                            walls.insert(point);
                        }
                    }
                    '@' => {
                        robot = (row as i32, col);
                    }
                    'O' => {
                        let rect = Rectangle::create(row as i32, col, width);
                        for point in rect.occupying_points() {
                            boxes.insert(point, rect);
                        }
                    }
                    _ => {}
                }
            }
        }
        Self {
            walls,
            boxes,
            robot,
        }
    }

    fn move_robot(&mut self, dir: (i32, i32)) {
        let mut dependents: Vec<Rectangle> = Vec::new();
        let mut queue: Vec<(i32, i32)> = Vec::new();
        queue.push((self.robot.0 + dir.0, self.robot.1 + dir.1));
        while let Some(current) = queue.pop() {
            if self.walls.contains(&current) {
                return;
            }
            if let Some(rect) = self.boxes.get(&current) {
                dependents.push(*rect);
                match dir {
                    (r, 0) => {
                        for c in rect.col..(rect.col + rect.width) {
                            queue.push((rect.row + r, c));
                        }
                    }
                    (0, -1) => queue.push((rect.row, rect.col - 1)), // Left
                    (0, 1) => queue.push((rect.row, rect.col + rect.width)), // Right
                    _ => unreachable!(),
                }
            }
        }
        self.robot = (self.robot.0 + dir.0, self.robot.1 + dir.1);
        for rect in &dependents {
            for point in rect.occupying_points() {
                self.boxes.remove(&point);
            }
        }
        for rect in dependents {
            let new_rect = rect.translate(dir);
            for point in new_rect.occupying_points() {
                self.boxes.insert(point, new_rect);
            }
        }
    }

    fn gps_sum(&self) -> i32 {
        self.boxes
            .values()
            .map(|r| (r.row, r.col))
            .sorted()
            .dedup()
            .map(|(r, c)| 100 * r + c)
            .sum()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let mut parts = input.split("\n\n");
        let grid = parts
            .next()
            .unwrap()
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let moves = parts
            .next()
            .unwrap()
            .chars()
            .filter_map(|dir| match dir {
                '^' => Some((-1, 0)),
                'v' => Some((1, 0)),
                '<' => Some((0, -1)),
                '>' => Some((0, 1)),
                _ => None,
            })
            .collect();
        Box::new(Day { grid, moves })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "\
            ##########\n\
            #..O..O.O#\n\
            #......O.#\n\
            #.OO..O.O#\n\
            #..O@..O.#\n\
            #O#..O...#\n\
            #O..O..O.#\n\
            #.OO.O.OO#\n\
            #....O...#\n\
            ##########\n\
            \n\
            <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\
            vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\
            ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\
            <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\
            ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\
            ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\
            >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\
            <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\
            ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\
            v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "10092");
    }

    #[test]
    fn test_part_1_example_2() {
        let input = "\
            ########\n\
            #..O.O.#\n\
            ##@.O..#\n\
            #...O..#\n\
            #.#.O..#\n\
            #...O..#\n\
            #......#\n\
            ########\n\
            \n\
            <^^>>>vv<v>>v<<";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "2028");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/15")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "1511865");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            ##########\n\
            #..O..O.O#\n\
            #......O.#\n\
            #.OO..O.O#\n\
            #..O@..O.#\n\
            #O#..O...#\n\
            #O..O..O.#\n\
            #.OO.O.OO#\n\
            #....O...#\n\
            ##########\n\
            \n\
            <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\
            vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\
            ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\
            <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\
            ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\
            ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\
            >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\
            <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\
            ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\
            v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "9021");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/15")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "1519991");
    }
}
