use crate::puzzle::Puzzle;

pub struct Day {
    grid: Vec<Vec<bool>>, // True if traversable, false if not.
    guard: (i32, i32),
}

impl Puzzle for Day {
    /// TODO
    ///
    /// Time complexity: O(m*n)
    /// Auxiliary space complexity: O(m*n)
    fn solve_part_1(&self) -> String {
        walk(&self.grid, self.guard.0, self.guard.1, true)
            .unwrap()
            .to_string()
    }

    /// TODO
    ///
    /// Time complexity: O(m^2 * n^2)
    /// Auxiliary space complexity: O(m*n)
    fn solve_part_2(&self) -> String {
        let mut count = 0;
        let mut grid = self.grid.clone();
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] {
                    grid[row][col] = false;
                    if walk(&grid, self.guard.0, self.guard.1, false).is_none() {
                        count += 1;
                    }
                    grid[row][col] = true;
                }
            }
        }
        count.to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let mut grid = Vec::new();
        let mut guard = (0, 0);
        for (row, line) in input.lines().enumerate() {
            let mut row_vec = Vec::new();
            for (col, ch) in line.chars().enumerate() {
                match ch {
                    '#' => row_vec.push(false),
                    '.' => row_vec.push(true),
                    '^' => {
                        row_vec.push(true);
                        guard = (row as i32, col as i32);
                    }
                    _ => unreachable!(),
                }
            }
            grid.push(row_vec);
        }
        Box::new(Day { grid, guard })
    }
}

fn walk(grid: &[Vec<bool>], mut row: i32, mut col: i32, perform_count: bool) -> Option<usize> {
    const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir = 0; // Initially pointing North.
    let mut seen = vec![vec![0u8; grid[0].len()]; grid.len()];
    loop {
        let mask = 1 << dir;
        if seen[row as usize][col as usize] & mask != 0 {
            return None;
        }
        seen[row as usize][col as usize] |= mask;
        let (drow, dcol) = DIRECTIONS[dir];
        let (next_row, next_col) = (row + drow, col + dcol);
        if !in_bounds(grid, next_row, next_col) {
            return if perform_count {
                Some(count_seen(&seen))
            } else {
                Some(0)
            };
        } else if !grid[next_row as usize][next_col as usize] {
            dir = (dir + 1) % 4;
        } else {
            row = next_row;
            col = next_col;
        }
    }
}

fn count_seen(seen: &[Vec<u8>]) -> usize {
    seen.iter()
        .map(|row| row.iter().filter(|&&cell| cell != 0).count())
        .sum()
}

fn in_bounds(grid: &[Vec<bool>], row: i32, col: i32) -> bool {
    row >= 0 && row < grid.len() as i32 && col >= 0 && col < grid[0].len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "\
            ....#.....\n\
            .........#\n\
            ..........\n\
            ..#.......\n\
            .......#..\n\
            ..........\n\
            .#..^.....\n\
            ........#.\n\
            #.........\n\
            ......#...";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "41");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/06")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "5564");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            ....#.....\n\
            .........#\n\
            ..........\n\
            ..#.......\n\
            .......#..\n\
            ..........\n\
            .#..^.....\n\
            ........#.\n\
            #.........\n\
            ......#...";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "6");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/06")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "1976");
    }
}
