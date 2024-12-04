use crate::puzzle::Puzzle;

pub struct Day {
    grid: Vec<Vec<char>>,
}

impl Puzzle for Day {
    /// We're given a grid of characters and asked to count the number of occurrences of a string
    /// in any direction.
    ///
    /// Time complexity: O(n*m)
    /// Auxiliary space complexity: O(1)
    fn solve_part_1(&self) -> String {
        let mut count = 0;
        for row in 0..self.grid.len() {
            for col in 0..self.grid[0].len() {
                count += count_xmas(&self.grid, row, col);
            }
        }
        count.to_string()
    }

    /// We're given a grid of characters and asked to count the number of occurrences of a
    /// particular pattern in the grid.
    ///
    /// Time complexity: O(n*m)
    /// Auxiliary space complexity: O(1)
    fn solve_part_2(&self) -> String {
        let mut count = 0;
        for row in 0..self.grid.len() {
            for col in 0..self.grid[0].len() {
                if is_x_mas(&self.grid, row as isize, col as isize) {
                    count += 1;
                }
            }
        }
        count.to_string()
    }
}

fn count_xmas(grid: &[Vec<char>], row: usize, col: usize) -> i32 {
    const DIRECTIONS: [(i8, i8); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    DIRECTIONS
        .iter()
        .filter(|&&dir| is_xmas(grid, row as isize, col as isize, dir))
        .count() as i32
}

fn is_xmas(grid: &[Vec<char>], mut row: isize, mut col: isize, dir: (i8, i8)) -> bool {
    const XMAS: &str = "XMAS";
    for expected in XMAS.chars() {
        if !in_bounds(grid, row, col) || grid[row as usize][col as usize] != expected {
            return false;
        }
        row += dir.0 as isize;
        col += dir.1 as isize;
    }
    true
}

fn is_x_mas(grid: &[Vec<char>], row: isize, col: isize) -> bool {
    if grid[row as usize][col as usize] != 'A' {
        return false;
    }
    let diagonals = [
        ((row - 1, col - 1), (row + 1, col + 1)),
        ((row - 1, col + 1), (row + 1, col - 1)),
    ];
    diagonals.iter().all(|&(loc1, loc2)| {
        if !in_bounds(grid, loc1.0, loc1.1) || !in_bounds(grid, loc2.0, loc2.1) {
            return false;
        }
        let val1 = grid[loc1.0 as usize][loc1.1 as usize];
        let val2 = grid[loc2.0 as usize][loc2.1 as usize];
        (val1 == 'M' && val2 == 'S') || (val1 == 'S' && val2 == 'M')
    })
}

fn in_bounds(grid: &[Vec<char>], row: isize, col: isize) -> bool {
    row >= 0 && (row as usize) < grid.len() && col >= 0 && (col as usize) < grid[0].len()
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let grid = input
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect();
        Box::new(Day { grid })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "\
            MMMSXXMASM\n\
            MSAMXMSMSA\n\
            AMXSXMAAMM\n\
            MSAMASMSMX\n\
            XMASAMXAMM\n\
            XXAMMXXAMA\n\
            SMSMSASXSS\n\
            SAXAMASAAA\n\
            MAMMMXMMMM\n\
            MXMXAXMASX";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_1(), "18");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/04")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "2560");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            MMMSXXMASM\n\
            MSAMXMSMSA\n\
            AMXSXMAAMM\n\
            MSAMASMSMX\n\
            XMASAMXAMM\n\
            XXAMMXXAMA\n\
            SMSMSASXSS\n\
            SAXAMASAAA\n\
            MAMMMXMMMM\n\
            MXMXAXMASX";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "9");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/04")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "1910");
    }
}
