use crate::puzzle::Puzzle;
use petgraph::prelude::NodeIndex;
use petgraph::Graph;

pub struct Day {
    bytes: Vec<(usize, usize)>,
}

impl Puzzle for Day {
    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_1(&self) -> String {
        let mut memory = Memory::new(70, self.bytes.clone());
        memory.apply_bytes(1024);
        memory.find_shortest_path().unwrap().to_string()
    }

    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_2(&self) -> String {
        let mut memory = Memory::new(70, self.bytes.clone());
        let (x, y) = memory.find_first_break();
        format!("{},{}", x, y)
    }
}

struct Memory {
    size: usize,
    graph: Vec<Vec<bool>>,
    bytes: Vec<(usize, usize)>,
    applied: usize,
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<usize>().unwrap();
            let y = parts.next().unwrap().parse::<usize>().unwrap();
            (x, y)
        })
        .collect()
}

impl Memory {
    fn new(size: usize, bytes: Vec<(usize, usize)>) -> Self {
        Self {
            size,
            graph: vec![vec![true; size + 1]; size + 1],
            bytes,
            applied: 0,
        }
    }

    fn apply_bytes(&mut self, new_applied: usize) {
        while self.applied < new_applied {
            let byte = self.bytes[self.applied];
            self.graph[byte.0][byte.1] = false;
            self.applied += 1;
        }
        while self.applied > new_applied {
            let byte = self.bytes[self.applied - 1];
            self.graph[byte.0][byte.1] = true;
            self.applied -= 1;
        }
    }

    fn find_first_break(&mut self) -> (usize, usize) {
        let mut low = 0;
        let mut high = self.bytes.len();
        while low < high {
            let mid = (low + high) / 2;
            self.apply_bytes(mid);
            if let Some(_) = self.find_shortest_path() {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        self.bytes[high - 1]
    }

    fn find_shortest_path(&self) -> Option<usize> {
        let mut graph = Graph::<(usize, usize), usize>::default();

        // Add nodes
        let mut nodes = vec![vec![NodeIndex::new(0); self.size + 1]; self.size + 1];
        for i in 0..=self.size {
            for j in 0..=self.size {
                if self.graph[i][j] {
                    nodes[i][j] =  graph.add_node((i, j));
                }
            }
        }

        // Add edges
        for i in 0..=self.size {
            for j in 0..=self.size {
                if self.graph[i][j] {
                    if i > 0 && self.graph[i - 1][j] {
                        graph.add_edge(nodes[i][j], nodes[i - 1][j], 1);
                    }
                    if j > 0 && self.graph[i][j - 1] {
                        graph.add_edge(nodes[i][j], nodes[i][j - 1], 1);
                    }
                    if i < self.size && self.graph[i + 1][j] {
                        graph.add_edge(nodes[i][j], nodes[i + 1][j], 1);
                    }
                    if j < self.size && self.graph[i][j + 1] {
                        graph.add_edge(nodes[i][j], nodes[i][j + 1], 1);
                    }
                }
            }
        }

        // Find the path
        let start = nodes[0][0];
        let end = nodes[self.size][self.size];
        let path = petgraph::algo::astar(&graph, start,
                                         |finish| finish == end,
                                         |e| *e.weight(),
                                         |n| {
                                             let &(i, j) = graph.node_weight(n).unwrap();
                                             2 * self.size - i - j
                                         });
        if let Some((distance, _)) = path {
            Some(distance)
        } else {
            None
        }
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        Box::new(Day { bytes: parse_input(input) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "\
            5,4\n\
            4,2\n\
            4,5\n\
            3,0\n\
            2,1\n\
            6,3\n\
            2,4\n\
            1,5\n\
            0,6\n\
            3,3\n\
            2,6\n\
            5,1\n\
            1,2\n\
            5,5\n\
            2,5\n\
            6,5\n\
            1,4\n\
            0,4\n\
            6,4\n\
            1,1\n\
            6,1\n\
            1,0\n\
            0,5\n\
            1,6\n\
            2,0";
        let bytes = parse_input(input);
        let mut memory = Memory::new(6, bytes);
        memory.apply_bytes(12);
        assert_eq!(memory.find_shortest_path(), Some(22));
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/18")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "384");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            5,4\n\
            4,2\n\
            4,5\n\
            3,0\n\
            2,1\n\
            6,3\n\
            2,4\n\
            1,5\n\
            0,6\n\
            3,3\n\
            2,6\n\
            5,1\n\
            1,2\n\
            5,5\n\
            2,5\n\
            6,5\n\
            1,4\n\
            0,4\n\
            6,4\n\
            1,1\n\
            6,1\n\
            1,0\n\
            0,5\n\
            1,6\n\
            2,0";
        let bytes = parse_input(input);
        let mut memory = Memory::new(6, bytes);
        assert_eq!(memory.find_first_break(), (6, 1));
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/18")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "36,10");
    }
}
