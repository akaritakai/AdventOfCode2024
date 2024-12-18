use crate::puzzle::Puzzle;
use itertools::iproduct;
use petgraph::algo::{all_simple_paths, has_path_connecting, DfsSpace};
use petgraph::graph::{Graph, NodeIndex};

pub struct Day {
    graph: Graph<(usize, usize, usize), ()>,
    starts: Vec<NodeIndex>,
    ends: Vec<NodeIndex>,
}

impl Puzzle for Day {
    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_1(&self) -> String {
        let mut space = DfsSpace::new(&self.graph);
        iproduct!(self.starts.iter(), self.ends.iter())
            .filter(|(&start, &end)| self.possibly_reachable(start, end))
            .map(|(&start, &end)| {
                if has_path_connecting(&self.graph, start, end, Some(&mut space)) {
                    1
                } else {
                    0
                }
            })
            .sum::<usize>()
            .to_string()
    }

    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_2(&self) -> String {
        iproduct!(self.starts.iter(), self.ends.iter())
            .filter(|(&start, &end)| self.possibly_reachable(start, end))
            .map(|(&start, &end)| {
                all_simple_paths::<Vec<_>, _>(&self.graph, start, end, 8, Some(8)).count()
            })
            .sum::<usize>()
            .to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let mut graph = Graph::new();
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let nodes = grid
            .iter()
            .enumerate()
            .map(|(row, line)| {
                line.iter()
                    .enumerate()
                    .map(|(col, &value)| graph.add_node((row, col, value)))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let starts = iproduct!(0..nodes.len(), 0..nodes[0].len())
            .filter(|&(row, col)| grid[row][col] == 0)
            .map(|(row, col)| nodes[row][col])
            .collect::<Vec<_>>();
        let ends = iproduct!(0..nodes.len(), 0..nodes[0].len())
            .filter(|&(row, col)| grid[row][col] == 9)
            .map(|(row, col)| nodes[row][col])
            .collect::<Vec<_>>();
        for (row, col) in iproduct!(0..nodes.len(), 0..nodes[0].len()) {
            if row > 0 && grid[row][col] + 1 == grid[row - 1][col] {
                graph.add_edge(nodes[row][col], nodes[row - 1][col], ());
            }
            if col > 0 && grid[row][col] + 1 == grid[row][col - 1] {
                graph.add_edge(nodes[row][col], nodes[row][col - 1], ());
            }
            if row + 1 < nodes.len() && grid[row][col] + 1 == grid[row + 1][col] {
                graph.add_edge(nodes[row][col], nodes[row + 1][col], ());
            }
            if col + 1 < nodes[0].len() && grid[row][col] + 1 == grid[row][col + 1] {
                graph.add_edge(nodes[row][col], nodes[row][col + 1], ());
            }
        }
        Box::new(Day {
            graph,
            starts,
            ends,
        })
    }

    fn possibly_reachable(&self, start: NodeIndex, end: NodeIndex) -> bool {
        let (start_row, start_col, _) = self.graph[start];
        let (end_row, end_col, _) = self.graph[end];
        let distance = (start_row as isize - end_row as isize).abs()
            + (start_col as isize - end_col as isize).abs();
        distance <= 9
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "\
            89010123\n\
            78121874\n\
            87430965\n\
            96549874\n\
            45678903\n\
            32019012\n\
            01329801\n\
            10456732";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "36");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/10")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "468");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            89010123\n\
            78121874\n\
            87430965\n\
            96549874\n\
            45678903\n\
            32019012\n\
            01329801\n\
            10456732";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "81");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/10")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "966");
    }
}
