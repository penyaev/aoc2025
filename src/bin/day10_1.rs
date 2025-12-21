use aoc_2025::utils::{input_file, read_lines};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

struct Item {
    value: u64,
    ops: u64,
    pos: usize,
}

impl Eq for Item {}

impl PartialEq<Self> for Item {
    fn eq(&self, other: &Self) -> bool {
        self.ops == other.ops
    }
}

impl PartialOrd<Self> for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.ops.cmp(&self.ops)
    }
}

struct Solver {
    queue: BinaryHeap<Item>,
    seen: HashSet<u64>,
    nums: Vec<u64>,
}

impl Solver {
    fn new(target: u64, nums: Vec<u64>) -> Self {
        let mut queue = BinaryHeap::new();
        queue.push(Item {
            value: target,
            ops: 0,
            pos: 0,
        });
        Self {
            queue,
            nums,
            seen: HashSet::new(),
        }
    }

    fn solve(&mut self) -> u64 {
        while !self.queue.is_empty() {
            let item = self.queue.pop().unwrap();
            if item.value == 0 {
                return item.ops;
            }
            if item.pos >= self.nums.len() {
                continue;
            }

            // don't use the number
            self.queue.push(Item {
                value: item.value,
                ops: item.ops,
                pos: item.pos + 1,
            });

            let next = item.value ^ self.nums[item.pos];
            if !self.seen.contains(&next) {
                self.seen.insert(next);
                self.queue.push(Item {
                    value: next,
                    ops: item.ops + 1,
                    pos: item.pos + 1,
                });
            }
        }
        panic!("failed to solve");
    }
}

fn main() {
    let input = read_lines(input_file(10, false)).expect("failed to read input");

    let mut result = 0u64;
    for line_result in input {
        let line = line_result.expect("failed to parse line");

        let parts: Vec<&str> = line.split(' ').collect();

        assert!(parts.len() >= 3);

        let bit_len = parts[0][1..parts[0].len() - 1].chars().count() as u64;
        let target = parts[0][1..parts[0].len() - 1]
            .chars()
            .map(|x| match x {
                '#' => 1u64,
                '.' => 0u64,
                _ => panic!("unexpected: [{}]", x),
            })
            .fold(0, |acc, x| acc * 2 + x);

        let nums: Vec<u64> = parts[1..parts.len() - 1]
            .iter()
            .map(|x| {
                x[1..x.len() - 1]
                    .split(',')
                    .map(|y| y.parse::<u64>().unwrap())
                    .fold(0, |acc, y| acc | (1 << (bit_len - y - 1)))
            })
            .collect();

        let mut solver = Solver::new(target, nums);
        result += solver.solve();
    }

    println!("{}", result);
}
