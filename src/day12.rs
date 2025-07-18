use crate::puzzle::Puzzle;
use geo::line_measures::LengthMeasurable;
use geo::{Area, BooleanOps, CoordsIter, Euclidean, MultiPolygon, polygon};
use itertools::iproduct;
use petgraph::unionfind::UnionFind;
use std::collections::HashMap;

pub struct Day {
    polygons: Vec<MultiPolygon>,
}

impl Puzzle for Day {
    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_1(&self) -> String {
        self.polygons
            .iter()
            .map(|multi_polygon| {
                let area = multi_polygon.unsigned_area() as usize;
                let perimeter = multi_polygon
                    .iter()
                    .map(|polygon| {
                        polygon.exterior().length(&Euclidean) as usize
                            + polygon
                                .interiors()
                                .iter()
                                .map(|interior| interior.length(&Euclidean) as usize)
                                .sum::<usize>()
                    })
                    .sum::<usize>();
                area * perimeter
            })
            .sum::<usize>()
            .to_string()
    }

    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_2(&self) -> String {
        self.polygons
            .iter()
            .map(|multi_polygon| {
                let area = multi_polygon.unsigned_area() as usize;
                let exterior_sides = multi_polygon.exterior_coords_iter().count() - 1;
                let interior_sides = multi_polygon
                    .iter()
                    .map(|polygon| {
                        polygon
                            .interiors()
                            .iter()
                            .map(|interior| interior.exterior_coords_iter().count() - 1)
                            .sum::<usize>()
                    })
                    .sum::<usize>();
                area * (exterior_sides + interior_sides)
            })
            .sum::<usize>()
            .to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let height = grid.len();
        let width = grid[0].len();

        let mut union_find: UnionFind<usize> = UnionFind::new(height * width);
        for (row, col) in iproduct!(0..height, 0..width) {
            if row > 0 && grid[row][col] == grid[row - 1][col] {
                union_find.union((row - 1) * width + col, row * width + col);
            }
            if col > 0 && grid[row][col] == grid[row][col - 1] {
                union_find.union(row * width + col - 1, row * width + col);
            }
        }

        let mut polygons: HashMap<usize, MultiPolygon> = HashMap::new();
        for (row, col) in iproduct!(0..height, 0..width) {
            let region = polygon![
                (x: col as f64, y: row as f64),
                (x: col as f64 + 1.0, y: row as f64),
                (x: col as f64 + 1.0, y: row as f64 + 1.0),
                (x: col as f64, y: row as f64 + 1.0),
            ];

            let index = union_find.find(row * width + col);
            if let Some(root) = polygons.get_mut(&index) {
                *root = root.union(&region);
            } else {
                polygons.insert(index, MultiPolygon::new(vec![region]));
            }
        }
        Box::new(Day {
            polygons: polygons.values().cloned().collect(),
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
            AAAA\n\
            BBCD\n\
            BBCC\n\
            EEEC";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "140");
    }

    #[test]
    fn test_part_1_example_2() {
        let input = "\
            OOOOO\n\
            OXOXO\n\
            OOOOO\n\
            OXOXO\n\
            OOOOO";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "772");
    }

    #[test]
    fn test_part_1_example_3() {
        let input = "\
            RRRRIICCFF\n\
            RRRRIICCCF\n\
            VVRRRCCFFF\n\
            VVRCCCJFFF\n\
            VVVVCJJCFE\n\
            VVIVCCJJEE\n\
            VVIIICJJEE\n\
            MIIIIIJJEE\n\
            MIIISIJEEE\n\
            MMMISSJEEE";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "1930");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/12")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "1396562");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            AAAA\n\
            BBCD\n\
            BBCC\n\
            EEEC";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "80");
    }

    #[test]
    fn test_part_2_example_2() {
        let input = "\
            OOOOO\n\
            OXOXO\n\
            OOOOO\n\
            OXOXO\n\
            OOOOO";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "436");
    }

    #[test]
    fn test_part_2_example_3() {
        let input = "\
            EEEEE\n\
            EXXXX\n\
            EEEEE\n\
            EXXXX\n\
            EEEEE";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "236");
    }

    #[test]
    fn test_part_2_example_4() {
        let input = "\
            AAAAAA\n\
            AAABBA\n\
            AAABBA\n\
            ABBAAA\n\
            ABBAAA\n\
            AAAAAA";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "368");
    }

    #[test]
    fn test_part_2_example_5() {
        let input = "\
            RRRRIICCFF\n\
            RRRRIICCCF\n\
            VVRRRCCFFF\n\
            VVRCCCJFFF\n\
            VVVVCJJCFE\n\
            VVIVCCJJEE\n\
            VVIIICJJEE\n\
            MIIIIIJJEE\n\
            MIIISIJEEE\n\
            MMMISSJEEE";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "1206");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/12")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "844132");
    }
}
