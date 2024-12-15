use crate::puzzle::Puzzle;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Day {
    antennas: HashMap<char, Vec<Point>>,
    height: i32,
    width: i32,
}

impl Puzzle for Day {
    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_1(&self) -> String {
        self.antennas
            .iter()
            .flat_map(|(_, antennas)| {
                antennas
                    .iter()
                    .combinations(2)
                    .flat_map(|pair| find_antinode_pair(pair[0], pair[1]))
                    .collect::<Vec<Point>>()
            })
            .filter(|antinode: &Point| {
                antinode.row >= 0
                    && antinode.row < self.height
                    && antinode.col >= 0
                    && antinode.col < self.width
            })
            .sorted_unstable()
            .dedup()
            .count()
            .to_string()
    }

    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_2(&self) -> String {
        self.antennas
            .iter()
            .flat_map(|(_, antennas)| {
                antennas
                    .iter()
                    .combinations(2)
                    .flat_map(|pair| {
                        find_all_antinode_pairs(pair[0], pair[1], self.height, self.width)
                    })
                    .collect::<Vec<Point>>()
            })
            .filter(|antinode: &Point| {
                antinode.row >= 0
                    && antinode.row < self.height
                    && antinode.col >= 0
                    && antinode.col < self.width
            })
            .sorted_unstable()
            .dedup()
            .count()
            .to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let mut height = 0;
        let mut width = 0;
        let mut antennas = HashMap::new();
        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c != '.' {
                    antennas.entry(c).or_insert_with(Vec::new).push(Point {
                        row: row as i32,
                        col: col as i32,
                    });
                }
                width = width.max(col as i32 + 1)
            }
            height = height.max(row as i32 + 1);
        }

        Box::new(Day {
            antennas,
            height,
            width,
        })
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
struct Point {
    row: i32,
    col: i32,
}

fn find_all_antinode_pairs(
    antenna_1: &Point,
    antenna_2: &Point,
    height: i32,
    width: i32,
) -> Vec<Point> {
    if antenna_1.col == antenna_2.col {
        // Vertical line.
        return (0..height)
            .map(|row| Point {
                row,
                col: antenna_1.col,
            })
            .collect();
    }
    if antenna_1.row == antenna_2.row {
        // Horizontal line.
        return (0..width)
            .map(|col| Point {
                row: antenna_1.row,
                col,
            })
            .collect();
    }
    let mut antinodes = Vec::new();
    let rise = antenna_2.row - antenna_1.row;
    let run = antenna_2.col - antenna_1.col;
    let mut point = Point {
        row: antenna_1.row,
        col: antenna_1.col,
    };
    while in_bounds(&point, height, width) {
        antinodes.push(point.clone());
        point.row += rise;
        point.col += run;
    }
    point.row -= rise;
    point.col -= run;
    while in_bounds(&point, height, width) {
        antinodes.push(point.clone());
        point.row -= rise;
        point.col -= run;
    }
    antinodes
}

fn find_antinode_pair(antenna_1: &Point, antenna_2: &Point) -> Vec<Point> {
    if antenna_1.col == antenna_2.col {
        // Vertical line.
        let x = std::cmp::min(antenna_1.row, antenna_2.row);
        let y = std::cmp::max(antenna_1.row, antenna_2.row);
        return vec![
            Point {
                row: 2 * y - x,
                col: antenna_1.col,
            },
            Point {
                row: 2 * x - y,
                col: antenna_1.col,
            },
        ];
    }
    if antenna_1.col == antenna_2.col {
        // Horizontal line.
        let x = std::cmp::min(antenna_1.col, antenna_2.col);
        let y = std::cmp::max(antenna_1.col, antenna_2.col);
        return vec![
            Point {
                row: antenna_1.row,
                col: 2 * y - x,
            },
            Point {
                row: antenna_1.row,
                col: 2 * x - y,
            },
        ];
    }
    vec![
        Point {
            row: 2 * antenna_2.row - antenna_1.row,
            col: 2 * antenna_2.col - antenna_1.col,
        },
        Point {
            row: 2 * antenna_1.row - antenna_2.row,
            col: 2 * antenna_1.col - antenna_2.col,
        },
    ]
}

fn in_bounds(point: &Point, height: i32, width: i32) -> bool {
    point.row >= 0 && point.row < height && point.col >= 0 && point.col < width
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "\
            ............\n\
            ........0...\n\
            .....0......\n\
            .......0....\n\
            ....0.......\n\
            ......A.....\n\
            ............\n\
            ............\n\
            ........A...\n\
            .........A..\n\
            ............\n\
            ............";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "14");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/08")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "359");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            ............\n\
            ........0...\n\
            .....0......\n\
            .......0....\n\
            ....0.......\n\
            ......A.....\n\
            ............\n\
            ............\n\
            ........A...\n\
            .........A..\n\
            ............\n\
            ............";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "34");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/08")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "1293");
    }
}
