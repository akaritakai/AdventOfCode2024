use crate::puzzle::Puzzle;
use itertools::iproduct;
use petgraph::algo::dijkstra;
use petgraph::graph::DiGraph;
use petgraph::prelude::NodeIndex;
use std::collections::HashMap;

pub struct Day {
    racetrack: Racetrack,
}

impl Puzzle for Day {
    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_1(&self) -> String {
        self.racetrack
            .find_cheats(2)
            .iter()
            .filter_map(|(&delta, &count)| if delta >= 100 { Some(count) } else { None })
            .sum::<usize>()
            .to_string()
    }

    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_2(&self) -> String {
        self.racetrack
            .find_cheats(20)
            .iter()
            .filter_map(|(&delta, &count)| if delta >= 100 { Some(count) } else { None })
            .sum::<usize>()
            .to_string()
    }
}

struct Racetrack {
    start: (usize, usize),
    end: (usize, usize),
    grid: Vec<Vec<bool>>,
}

impl Racetrack {
    fn create(input: &str) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let grid: Vec<Vec<bool>> = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, ch)| {
                        match ch {
                            '#' => false, // Not traversable
                            '.' => true,  // Traversable
                            'S' => {
                                start = (row, col);
                                true // Start is traversable
                            }
                            'E' => {
                                end = (row, col);
                                true // End is traversable
                            }
                            _ => unreachable!(),
                        }
                    })
                    .collect()
            })
            .collect();
        Racetrack { start, end, grid }
    }

    fn num_rows(&self) -> usize {
        self.grid.len()
    }

    fn num_cols(&self) -> usize {
        self.grid[0].len()
    }

    fn neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::with_capacity(4);
        if row > 0 {
            neighbors.push((row - 1, col));
        }
        if col > 0 {
            neighbors.push((row, col - 1));
        }
        if row < self.num_rows() - 1 {
            neighbors.push((row + 1, col));
        }
        if col < self.num_cols() - 1 {
            neighbors.push((row, col + 1));
        }
        neighbors
    }

    fn distance_map(&self, start: (usize, usize)) -> Vec<Vec<usize>> {
        let mut graph = DiGraph::<(usize, usize), usize>::default();
        let mut indices = vec![vec![NodeIndex::new(0); self.num_cols()]; self.num_rows()];

        // Add nodes
        for (a, b) in iproduct!(0..self.num_rows(), 0..self.num_cols()) {
            if self.grid[a][b] {
                indices[a][b] = graph.add_node((a, b));
            }
        }

        // Add edges
        for (a, b) in iproduct!(0..self.num_rows(), 0..self.num_cols()) {
            if !self.grid[a][b] {
                continue;
            }
            for (c, d) in self.neighbors(a, b) {
                if self.grid[c][d] {
                    graph.add_edge(indices[a][b], indices[c][d], 1);
                }
            }
        }

        let start = indices[start.0][start.1];
        let result = dijkstra(&graph, start, None, |e| *e.weight());
        let mut distances = vec![vec![usize::MAX; self.num_cols()]; self.num_rows()];
        for (&node, &dist) in result.iter() {
            let &(row, col) = graph.node_weight(node).unwrap();
            distances[row][col] = dist;
        }
        distances
    }

    fn manhatten_distance(&self, a: (usize, usize), b: (usize, usize)) -> usize {
        (a.0 as isize - b.0 as isize).abs() as usize + (a.1 as isize - b.1 as isize).abs() as usize
    }

    fn find_cheats(&self, cheat_time: usize) -> HashMap<usize, usize> {
        let start_distances = self.distance_map(self.start);
        let end_distances = self.distance_map(self.end);
        let honorable_distance = start_distances[self.end.0][self.end.1];
        let mut deltas: HashMap<usize, usize> = HashMap::new();
        for (a, b) in iproduct!(0..self.num_rows(), 0..self.num_cols()) {
            let start_to_cheat_distance = start_distances[a][b];
            if start_to_cheat_distance == usize::MAX {
                continue;
            }
            for (c, d) in iproduct!(0..self.num_rows(), 0..self.num_cols()) {
                let cheat_to_end_distance = end_distances[c][d];
                if cheat_to_end_distance == usize::MAX {
                    continue;
                }
                let cheat_distance = self.manhatten_distance((a, b), (c, d));
                if cheat_distance > cheat_time {
                    continue;
                }
                let distance = start_to_cheat_distance + cheat_distance + cheat_to_end_distance;
                if distance < honorable_distance {
                    let delta = honorable_distance - distance;
                    let entry = deltas.entry(delta).or_insert(0);
                    *entry += 1;
                }
            }
        }
        deltas
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let racetrack = Racetrack::create(input);
        Box::new(Day { racetrack })
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
            #...#...#.....#\n\
            #.#.#.#.#.###.#\n\
            #S#...#.#.#...#\n\
            #######.#.#.###\n\
            #######.#.#...#\n\
            #######.#.###.#\n\
            ###..E#...#...#\n\
            ###.#######.###\n\
            #...###...#...#\n\
            #.#####.#.###.#\n\
            #.#...#.#.#...#\n\
            #.#.#.#.#.#.###\n\
            #...#...#...###\n\
            ###############";
        let racetrack = Racetrack::create(input);
        let deltas = racetrack.find_cheats(2);
        assert_eq!(*deltas.get(&2).unwrap(), 14);
        assert_eq!(*deltas.get(&4).unwrap(), 14);
        assert_eq!(*deltas.get(&6).unwrap(), 2);
        assert_eq!(*deltas.get(&8).unwrap(), 4);
        assert_eq!(*deltas.get(&10).unwrap(), 2);
        assert_eq!(*deltas.get(&12).unwrap(), 3);
        assert_eq!(*deltas.get(&20).unwrap(), 1);
        assert_eq!(*deltas.get(&36).unwrap(), 1);
        assert_eq!(*deltas.get(&38).unwrap(), 1);
        assert_eq!(*deltas.get(&40).unwrap(), 1);
        assert_eq!(*deltas.get(&64).unwrap(), 1);
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/20")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "1409");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            ###############\n\
            #...#...#.....#\n\
            #.#.#.#.#.###.#\n\
            #S#...#.#.#...#\n\
            #######.#.#.###\n\
            #######.#.#...#\n\
            #######.#.###.#\n\
            ###..E#...#...#\n\
            ###.#######.###\n\
            #...###...#...#\n\
            #.#####.#.###.#\n\
            #.#...#.#.#...#\n\
            #.#.#.#.#.#.###\n\
            #...#...#...###\n\
            ###############";
        let racetrack = Racetrack::create(input);
        let deltas = racetrack.find_cheats(20);
        assert_eq!(*deltas.get(&50).unwrap(), 32);
        assert_eq!(*deltas.get(&52).unwrap(), 31);
        assert_eq!(*deltas.get(&54).unwrap(), 29);
        assert_eq!(*deltas.get(&56).unwrap(), 39);
        assert_eq!(*deltas.get(&58).unwrap(), 25);
        assert_eq!(*deltas.get(&60).unwrap(), 23);
        assert_eq!(*deltas.get(&62).unwrap(), 20);
        assert_eq!(*deltas.get(&64).unwrap(), 19);
        assert_eq!(*deltas.get(&66).unwrap(), 12);
        assert_eq!(*deltas.get(&68).unwrap(), 14);
        assert_eq!(*deltas.get(&70).unwrap(), 12);
        assert_eq!(*deltas.get(&72).unwrap(), 22);
        assert_eq!(*deltas.get(&74).unwrap(), 4);
        assert_eq!(*deltas.get(&76).unwrap(), 3);
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/20")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "1012821");
    }
}
