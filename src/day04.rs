use crate::puzzle::Puzzle;

pub struct Day {
    grid: Vec<Vec<char>>,
}

impl Puzzle for Day {
    fn solve_part_1(&self) -> String {
        let mut count = 0;
        for row in 0..self.grid.len() {
            for col in 0..self.grid[0].len() {
                count += count_xmas(&self.grid, row, col);
            }
        }
        count.to_string()
    }

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

static XMAS: &str = "XMAS";

fn count_xmas(grid: &[Vec<char>], row: usize, col: usize) -> i32 {
    let mut count = 0;
    for drow in -1..=1 {
        for dcol in -1..=1 {
            if drow == 0 && dcol == 0 {
                continue;
            }
            if is_xmas(grid, row as isize, col as isize, (drow, dcol)) {
                count += 1;
            }
        }
    }
    count
}

fn is_xmas(grid: &[Vec<char>], mut row: isize, mut col: isize, dir: (i8, i8)) -> bool {
    let mut i = 0;
    while in_bounds(grid, row, col) && i < XMAS.len() {
        if grid[row as usize][col as usize] != XMAS.chars().nth(i).unwrap() {
            return false;
        }
        row += dir.0 as isize;
        col += dir.1 as isize;
        i += 1;
    }
    i == 4
}

fn is_x_mas(grid: &[Vec<char>], row: isize, col: isize) -> bool {
    if grid[row as usize][col as usize] != 'A'
        || !in_bounds(grid, row - 1, col - 1)
        || !in_bounds(grid, row - 1, col + 1)
        || !in_bounds(grid, row + 1, col - 1)
        || !in_bounds(grid, row + 1, col + 1)
    {
        return false;
    }
    let up_left = grid[(row - 1) as usize][(col - 1) as usize];
    let up_right = grid[(row - 1) as usize][(col + 1) as usize];
    let down_left = grid[(row + 1) as usize][(col - 1) as usize];
    let down_right = grid[(row + 1) as usize][(col + 1) as usize];
    let mut matches = 0;
    if up_left == 'M' && down_right == 'S' {
        matches += 1;
    }
    if up_left == 'S' && down_right == 'M' {
        matches += 1;
    }
    if up_right == 'M' && down_left == 'S' {
        matches += 1;
    }
    if up_right == 'S' && down_left == 'M' {
        matches += 1;
    }
    matches == 2
}

fn in_bounds(grid: &[Vec<char>], row: isize, col: isize) -> bool {
    row >= 0 && row < grid.len() as isize && col >= 0 && col < grid[0].len() as isize
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
