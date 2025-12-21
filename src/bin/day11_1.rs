use aoc_2025::utils::{input_file, read_lines};
use std::collections::{HashMap, VecDeque};


struct Solver {
    graph: HashMap<String, Vec<String>>,
}

impl Solver {
    fn add_node(&mut self, from: String, to: Vec<String>) {
        self.graph.insert(from, to);
    }

    fn new() -> Self {
        Self { graph: HashMap::new() }
    }

    fn solve(&self) -> u64 {
        let mut queue: VecDeque<&String> = VecDeque::new();

        let start = String::from("you");
        queue.push_back(&start);
        let mut cnt = HashMap::new();


        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            cnt.entry(node).and_modify(|c| *c += 1).or_insert(1u64);

            let next_nodes = self.graph.get(node);
            if next_nodes.is_none() { continue; }
            for next in next_nodes.unwrap() {
                queue.push_back(next);
            }
        }

        *cnt.get(&String::from("out")).unwrap()
    }
}

fn main() {
    let input = read_lines(input_file(11, false)).expect("failed to read input");

    let mut solver = Solver::new();

    for line_result in input {
        let line = line_result.expect("failed to parse line");

        let parts: Vec<&str> = line.split(' ').collect();

        assert!(parts.len() >= 2);

        let from = parts[0].trim_matches(':').to_string();
        let to: Vec<_> = parts[1..].iter().map(|x| x.to_string()).collect();

        solver.add_node(from, to);
    }

    println!("{}", solver.solve());
}
