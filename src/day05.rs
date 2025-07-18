use crate::puzzle::Puzzle;
use petgraph::Graph;
use petgraph::algo::toposort;
use petgraph::graph::DiGraph;
use std::collections::HashMap;

pub struct Day {
    dependencies: Vec<(i32, i32)>,
    print_jobs: Vec<Vec<i32>>,
}

impl Puzzle for Day {
    /// We're given a list of nodes as well as their edges and asked to return the middle node of
    /// the list if it's in topological order.
    ///
    /// Time complexity: O(V+E)
    /// Auxiliary space complexity: O(V+E)
    fn solve_part_1(&self) -> String {
        self.process_jobs(true).to_string()
    }

    /// We're given a list of nodes as well as their edges and asked to return the middle node of
    /// the list as if it were in topological order.
    ///
    /// Time complexity: O(V+E)
    /// Auxiliary space complexity: O(V+E)
    fn solve_part_2(&self) -> String {
        self.process_jobs(false).to_string()
    }
}

fn create_graph(dependencies: &[(i32, i32)], print_job: &[i32]) -> Graph<i32, ()> {
    let mut graph = DiGraph::new();
    let pages: HashMap<i32, _> = print_job
        .iter()
        .map(|&page| (page, graph.add_node(page)))
        .collect();
    for (left, right) in dependencies {
        if let (Some(&l), Some(&r)) = (pages.get(left), pages.get(right)) {
            graph.add_edge(l, r, ());
        }
    }
    graph
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let mut parts = input.split("\n\n");
        let dependencies = parts
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let mut parts = line.split('|');
                let left = parts.next().unwrap().parse().unwrap();
                let right = parts.next().unwrap().parse().unwrap();
                (left, right)
            })
            .collect();
        let print_jobs = parts
            .next()
            .unwrap()
            .lines()
            .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect())
            .collect();
        Box::new(Day {
            dependencies,
            print_jobs,
        })
    }

    fn process_jobs(&self, match_correct: bool) -> i32 {
        self.print_jobs
            .iter()
            .filter_map(|job| {
                let graph = create_graph(&self.dependencies, job);
                let sorted = toposort(&graph, None)
                    .ok()
                    .map(|order| order.into_iter().map(|i| graph[i]).collect::<Vec<_>>())?;
                if match_correct == (job == &sorted) {
                    Some(sorted[sorted.len() / 2])
                } else {
                    None
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "\
            47|53\n\
            97|13\n\
            97|61\n\
            97|47\n\
            75|29\n\
            61|13\n\
            75|53\n\
            29|13\n\
            97|29\n\
            53|29\n\
            61|53\n\
            97|53\n\
            61|29\n\
            47|13\n\
            75|47\n\
            97|75\n\
            47|61\n\
            75|61\n\
            47|29\n\
            75|13\n\
            53|13\n\
            \n\
            75,47,61,53,29\n\
            97,61,53,29,13\n\
            75,29,13\n\
            75,97,47,61,53\n\
            61,13,29\n\
            97,13,75,29,47";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "143");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/05")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "5374");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            47|53\n\
            97|13\n\
            97|61\n\
            97|47\n\
            75|29\n\
            61|13\n\
            75|53\n\
            29|13\n\
            97|29\n\
            53|29\n\
            61|53\n\
            97|53\n\
            61|29\n\
            47|13\n\
            75|47\n\
            97|75\n\
            47|61\n\
            75|61\n\
            47|29\n\
            75|13\n\
            53|13\n\
            \n\
            75,47,61,53,29\n\
            97,61,53,29,13\n\
            75,29,13\n\
            75,97,47,61,53\n\
            61,13,29\n\
            97,13,75,29,47";
        let puzzle = Day::create(input);
        assert_eq!(puzzle.solve_part_2(), "123");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/05")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "4260");
    }
}
