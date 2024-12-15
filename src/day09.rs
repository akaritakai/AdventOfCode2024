use crate::puzzle::Puzzle;

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_1(&self) -> String {
        // Build the disk
        let mut ids = 0;
        let mut disk = Vec::new();
        for (i, c) in self.input.chars().enumerate() {
            for _ in 0..c.to_digit(10).unwrap() {
                if i % 2 == 0 {
                    disk.push(Block::File(ids));
                } else {
                    disk.push(Block::Free);
                }
            }
            if i % 2 == 0 {
                ids += 1;
            }
        }

        // Perform the defrag
        let mut j: isize = disk.len() as isize - 1;
        let mut i: isize = 0;
        loop {
            while j >= 0 && disk[j as usize] == Block::Free {
                j -= 1;
            }
            while i < disk.len() as isize && disk[i as usize] != Block::Free {
                i += 1;
            }
            if i >= j {
                break; // No more moves
            }
            disk.swap(i as usize, j as usize);
        }

        // Calculate the checksum
        let mut checksum = 0u64;
        for (i, block) in disk.iter().enumerate() {
            if let Block::File(id) = block {
                checksum += i as u64 * (*id) as u64;
            }
        }
        checksum.to_string()
    }

    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_2(&self) -> String {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Block {
    File(i32),
    Free,
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        Box::new(Day {
            input: input.trim().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "2333133121414131402";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "1928");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/09")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "6385338159127");
    }
}
