use aoc_2025::utils::{input_file, read_lines};
use indexmap::IndexSet;
use std::collections::{HashMap, HashSet, VecDeque};

struct Solver {
    graph: HashMap<String, Vec<String>>,
    incoming_edges: HashMap<String, usize>,
}

impl Solver {
    fn add_node(&mut self, from: String, to: Vec<String>) {
        to.iter()
            .for_each(|x| *self.incoming_edges.entry(x.clone()).or_insert(0) += 1);
        self.graph.insert(from, to);
    }

    fn new() -> Self {
        Self {
            graph: HashMap::new(),
            incoming_edges: HashMap::new(),
        }
    }

    fn solve(&self, start: &String, end: &String) -> usize {
        let mut queue: VecDeque<(&String, usize)> = VecDeque::new();
        let mut incoming_edges_used = HashMap::new();
        let mut secondary_queue = IndexSet::new();
        let mut visited = HashSet::new();

        let mut cnt = HashMap::new();
        cnt.insert(start, 1);
        secondary_queue.insert(start);

        while !secondary_queue.is_empty() {
            let node = secondary_queue.shift_remove_index(0).unwrap();
            let next_nodes = self.graph.get(node);
            if visited.contains(node) {
                continue;
            }
            visited.insert(node);
            if next_nodes.is_some() {
                for next in next_nodes.unwrap() {
                    let c = *cnt.get(node).unwrap();
                    // println!("(sec) {} -> {}: {}", node, next, c);
                    queue.push_back((next, c));
                }
            }

            // println!("----");
            while !queue.is_empty() {
                let (node, node_cnt) = queue.pop_front().unwrap();
                cnt.entry(node)
                    .and_modify(|c| *c += node_cnt)
                    .or_insert(node_cnt);
                incoming_edges_used
                    .entry(node)
                    .and_modify(|c| *c += 1)
                    .or_insert(1usize);
                secondary_queue.insert(node);

                if *node != *start
                    && (incoming_edges_used.get(node).unwrap_or(&0)
                        != self.incoming_edges.get(node).unwrap_or(&0))
                {
                    continue;
                }
                secondary_queue.shift_remove(node);
                if visited.contains(node) {
                    continue;
                }
                visited.insert(node);
                let next_nodes = self.graph.get(node);
                if next_nodes.is_none() {
                    continue;
                }
                for next in next_nodes.unwrap() {
                    let c = *cnt.get(node).unwrap();
                    // println!("(mai) {} -> {}: {}", node, next, c);
                    queue.push_back((next, c));
                }
            }
        }


        *cnt.get(end).unwrap_or(&0)
    }

    fn prune_unreachable(&mut self, start: &String) {
        let mut queue: VecDeque<&String> = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(start);

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            if visited.contains(node) {
                continue;
            }
            visited.insert(node.clone());
            let next_nodes = self.graph.get(node);
            if next_nodes.is_some() {
                for next in next_nodes.unwrap() {
                    if !visited.contains(next) {
                        queue.push_back(next);
                    }
                }
            }
        }

        self.graph.iter().enumerate().for_each(|(i, (from, to))| {
            if visited.contains(from) {
                return;
            }
            to.iter().for_each(|x| {
                self.incoming_edges.entry(x.clone()).and_modify(|c| *c -= 1);
            });

        });
        self.graph.retain(|k, _| visited.contains(k));
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

    // svr -> fft: 12371
    // fft -> dac: 11740656
    // dac -> fft: 0
    // dac -> out: 3263
    // result: 473930047491888

    let original_len = solver.graph.len();
    solver.prune_unreachable(&"fft".into());
    println!("pruned {:?} nodes", solver.graph.len().abs_diff(original_len));
    println!("{}", solver.solve(&"fft".into(), &"dac".into()));
    // let a = solver.solve(&"svr".into(), &"fft".into());
    // let b = solver.solve(&"fft".into(), &"dac".into());
    // let c = solver.solve(&"svr".into(), &"dac".into());
    //
    // println!("{}", 12371u64*11740656u64*3263u64);

    // println!("{}",
    //          solver.solve(&"svr".into(), &"fft".into()) * solver.solve(&"fft".into(), &"dac".into()) * solver.solve(&"dac".into(), &"out".into())
    // );
}
