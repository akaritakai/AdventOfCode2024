use crate::puzzle::Puzzle;
use rangemap::RangeMap;
use std::ops::Range;

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_1(&self) -> String {
        let mut disk = Disk::create_from_map(&self.input);
        disk.defrag_blocks();
        disk.checksum().to_string()
    }

    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_2(&self) -> String {
        let mut disk = Disk::create_from_map(&self.input);
        disk.defrag_files();
        disk.checksum().to_string()
    }
}

struct Disk {
    files: RangeMap<usize, usize>,
    size: Range<usize>,
}

impl Disk {
    fn create_from_map(input: &str) -> Self {
        let mut size = 0;
        let mut files = RangeMap::new();
        for (i, c) in input.chars().enumerate() {
            let c = c.to_digit(10).unwrap() as usize;
            if c == 0 {
                continue;
            }
            if i % 2 == 0 {
                files.insert(size..size+c, i / 2);
            }
            size += c;
        }
        Self { files, size: 0..size }
    }

    fn defrag_blocks(&mut self) {
        loop {
            let empty = self.files.gaps(&self.size).next();
            let file = self.files.last_range_value().map(|(range, &id)| (range.clone(), id));
            if empty.is_none() || file.is_none() {
                return;
            }
            let empty = empty.unwrap();
            let (file, id) = file.unwrap();
            if file.start <= empty.start {
                return;
            }
            if file.len() <= empty.len() {
                self.files.remove(file.clone()); // Remove the old file
                self.files.insert(empty.start..empty.start+file.len(), id);
            } else {
                self.files.remove(file.end-empty.len()..file.end);
                self.files.insert(empty, id);
            }
        }
    }

    fn defrag_files(&mut self) {
        for (range, &id) in self.files.clone().iter().rev() {
            for gap in self.files.gaps(&self.size) {
                if gap.start > range.start {
                    break;
                }
                if gap.len() >= range.len() {
                    self.files.remove(range.clone());
                    self.files.insert(gap.start..gap.start+range.len(), id);
                    break;
                }
            }
        }
    }

    fn checksum(&self) -> usize {
        self.files.iter().map(|(range, &id)| {
            let length = range.end - range.start;
            id * length * (2 * range.start + length - 1) / 2
        }).sum()
    }
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

    #[test]
    fn test_part_2_example_1() {
        let input = "2333133121414131402";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "2858");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/09")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "6415163624282");
    }
}
