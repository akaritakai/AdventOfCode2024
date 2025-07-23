use crate::puzzle::Puzzle;
use petgraph::graph::NodeIndex;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use petgraph::algo::dijkstra;
use petgraph::prelude::EdgeRef;
use petgraph::visit::Reversed;

pub struct Day {
    start: NodeIndex,
    end: Vec<NodeIndex>,
    graph: petgraph::Graph<Vertex, i32>,
}

impl Puzzle for Day {
    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_1(&self) -> String {
        // Find the shortest path from start to end.
        let paths = dijkstra(&self.graph, self.start, None, |e| *e.weight());
        self.end.iter()
            .filter_map(|&end| paths.get(&end))
            .min()
            .unwrap()
            .to_string()
    }

    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_2(&self) -> String {
        let dist_s = dijkstra(&self.graph, self.start, None, |e| *e.weight());
        let &target_cost = self.end.iter()
            .filter_map(|&end| dist_s.get(&end))
            .min()
            .unwrap();
        let rev = Reversed(&self.graph);
        let dist_e = self.end.iter()
            .map(|&end| dijkstra(&rev, end, None, |e| *e.weight()))
            .collect_vec();
        
        // Find all nodes...
        let mut winners = HashSet::new();
        for v in self.graph.node_indices() {
            if let Some(s_to_v) = dist_s.get(&v) {
                for d in &dist_e {
                    if let Some(v_to_e) = d.get(&v) {
                        if s_to_v + v_to_e == target_cost {
                            if let Some(vertex) = self.graph.node_weight(v) {
                                winners.insert((vertex.row, vertex.col));
                            }
                        }
                    }
                }
            }
        }
        winners.iter().count().to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vertex {
    row: i32,
    col: i32,
    facing: (i32, i32),
}

impl Vertex {
    fn clockwise(&self) -> Self {
        Self {
            row: self.row,
            col: self.col,
            facing: (self.facing.1, -self.facing.0),
        }
    }

    fn counter_clockwise(&self) -> Self {
        Self {
            row: self.row,
            col: self.col,
            facing: (-self.facing.1, self.facing.0),
        }
    }

    fn forward(&self) -> Self {
        Self {
            row: self.row + self.facing.0,
            col: self.col + self.facing.1,
            facing: self.facing,
        }
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut graph = petgraph::Graph::new();
        let mut vertex_map = HashMap::new();
        let mut add_vertex = |row: i32, col: i32| {
            for &facing in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let vertex = Vertex { row, col, facing };
                let index = graph.add_node(vertex);
                vertex_map.insert(vertex, index);
            }
        };
        let mut start = (0, 0);
        let mut end = (0, 0);
        for (row, line) in grid.iter().enumerate() {
            for (col, ch) in line.iter().enumerate() {
                match ch {
                    'S' => {
                        start = (row as i32, col as i32);
                        add_vertex(start.0, start.1);
                    }
                    'E' => {
                        end = (row as i32, col as i32);
                        add_vertex(end.0, end.1);
                    }
                    '.' => {
                        add_vertex(row as i32, col as i32);
                    }
                    _ => {}
                }
            }
        }

        // Handle facing edges.
        for (vertex, &src) in &vertex_map {
            // Clockwise turn.
            if let Some(&dst) = vertex_map.get(&vertex.clockwise()) {
                graph.add_edge(src, dst, 1000);
            }
            if let Some(&dst) = vertex_map.get(&vertex.counter_clockwise()) {
                graph.add_edge(src, dst, 1000);
            }
        }

        // Handle movement edges.
        for (vertex, &src) in &vertex_map {
            if let Some(&dst) = vertex_map.get(&vertex.forward()) {
                graph.add_edge(src, dst, 1);
            }
        }

        Box::new(Day {
            start: vertex_map[&Vertex {
                row: start.0,
                col: start.1,
                facing: (0, 1),
            }],
            end: vertex_map.iter()
                .filter(|v| v.0.row == end.0 && v.0.col == end.1)
                .map(|(_, &index)| index)
                .collect::<Vec<_>>(),
            graph,
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
            ###############\n\
            #.......#....E#\n\
            #.#.###.#.###.#\n\
            #.....#.#...#.#\n\
            #.###.#####.#.#\n\
            #.#.#.......#.#\n\
            #.#.#####.###.#\n\
            #...........#.#\n\
            ###.#.#####.#.#\n\
            #...#.....#.#.#\n\
            #.#.#.###.#.#.#\n\
            #.....#...#.#.#\n\
            #.###.#.#.#.#.#\n\
            #S..#.....#...#\n\
            ###############";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "7036");
    }

    #[test]
    fn test_part_1_example_2() {
        let input = "\
            #################\n\
            #...#...#...#..E#\n\
            #.#.#.#.#.#.#.#.#\n\
            #.#.#.#...#...#.#\n\
            #.#.#.#.###.#.#.#\n\
            #...#.#.#.....#.#\n\
            #.#.#.#.#.#####.#\n\
            #.#...#.#.#.....#\n\
            #.#.#####.#.###.#\n\
            #.#.#.......#...#\n\
            #.#.###.#####.###\n\
            #.#.#...#.....#.#\n\
            #.#.#.#####.###.#\n\
            #.#.#.........#.#\n\
            #.#.#.#########.#\n\
            #S#.............#\n\
            #################";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "11048");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/16")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "93436");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            ###############\n\
            #.......#....E#\n\
            #.#.###.#.###.#\n\
            #.....#.#...#.#\n\
            #.###.#####.#.#\n\
            #.#.#.......#.#\n\
            #.#.#####.###.#\n\
            #...........#.#\n\
            ###.#.#####.#.#\n\
            #...#.....#.#.#\n\
            #.#.#.###.#.#.#\n\
            #.....#...#.#.#\n\
            #.###.#.#.#.#.#\n\
            #S..#.....#...#\n\
            ###############";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "45");
    }

    #[test]
    fn test_part_2_example_2() {
        let input = "\
            #################\n\
            #...#...#...#..E#\n\
            #.#.#.#.#.#.#.#.#\n\
            #.#.#.#...#...#.#\n\
            #.#.#.#.###.#.#.#\n\
            #...#.#.#.....#.#\n\
            #.#.#.#.#.#####.#\n\
            #.#...#.#.#.....#\n\
            #.#.#####.#.###.#\n\
            #.#.#.......#...#\n\
            #.#.###.#####.###\n\
            #.#.#...#.....#.#\n\
            #.#.#.#####.###.#\n\
            #.#.#.........#.#\n\
            #.#.#.#########.#\n\
            #S#.............#\n\
            #################";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "64");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/16")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "486");
    }
}
 