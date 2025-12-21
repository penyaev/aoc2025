use aoc_2025::utils::{input_file, read_lines};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::ops::Add;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct Value {
    digits: Vec<u8>,
}

impl Value {
    fn new(len: usize) -> Self {
        Self {
            digits: vec![0; len],
        }
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            digits: self.digits.iter().zip(other.digits.iter()).map(|(x, y)| x + y).collect()
        }
    }
}


impl PartialOrd<Self> for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut eq_cnt = 0;
        for i in 0..self.digits.len() {
            match self.digits[i].cmp(&other.digits[i]) {
                Ordering::Greater => return Some(Ordering::Greater),
                Ordering::Equal => eq_cnt += 1,
                Ordering::Less => continue,
            };
        }
        if eq_cnt == self.digits.len() {
            return Some(Ordering::Equal);
        }
        Some(Ordering::Less)
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
struct Item {
    value: Value,
    ops: usize,
    min_dist_to_target: usize,
}
impl PartialOrd<Self> for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.ops + self.min_dist_to_target).cmp(&(other.ops + other.min_dist_to_target)).reverse()
    }
}

struct Solver {
    queue: BinaryHeap<Item>,
    nums: Vec<Value>,
    target: Value,
    seen: HashSet<Value>,
    memo: HashMap<usize, Vec<Value>>,
}

impl Solver {
    fn new(nums: Vec<Value>, target: Value) -> Self {
        Self {
            queue: BinaryHeap::new(),
            nums,
            target,
            seen: HashSet::new(),
            memo: HashMap::new(),
        }
    }

    fn nums_with_digit(&mut self, pos: usize) -> &Vec<Value> {
        if !self.memo.contains_key(&pos) {
            let res = self
                .nums
                .iter()
                .filter(|x| x.digits[pos] == 1)
                .filter(|x| x.digits[0..pos].iter().all(|x| *x == 0))
                .cloned()
                .collect();
            self.memo.insert(pos, res);
        }
        self.memo.get(&pos).unwrap()
    }

    fn digit_reached(&self, pos: usize, value: &Value) -> bool {
        value.digits[pos] == self.target.digits[pos]
    }

    fn min_dist_to_target(&self, value: &Value) -> usize {
        value
            .digits
            .iter()
            .enumerate()
            .map(|(i, x)| x.abs_diff(self.target.digits[i]) as usize)
            .max()
            .unwrap()
    }

    fn solve(&mut self) -> usize {
        self.queue.clear();
        let init_value = Value::new(self.target.digits.len());
        let min_dist = self.min_dist_to_target(&init_value);
        self.queue.push(Item {
            ops: 0,
            value: init_value,
            min_dist_to_target: min_dist,
        });

        while !self.queue.is_empty() {
            let item = self.queue.pop().unwrap();

            if item.value > self.target {
                continue;
            }
            if item.value == self.target {
                return item.ops;
            }
            if self.seen.contains(&item.value) {
                continue;
            }
            self.seen.insert(item.value.clone());

            for num in &self.nums {
                let new_value = item.value.add(num);
                if new_value > self.target {
                    continue;
                }
                self.queue.push(Item{
                    min_dist_to_target: self.min_dist_to_target(&new_value),
                    ops: item.ops + 1,
                    value: new_value
                });
            }
        }

        panic!("failed to solve");
    }
}

fn main() {
    let input = read_lines(input_file(10, true)).expect("failed to read input");

    let mut result = 0;
    for line_result in input {
        let line = line_result.expect("failed to parse line");

        let parts: Vec<&str> = line.split(' ').collect();

        assert!(parts.len() >= 3);

        let bit_len = parts[0][1..parts[0].len() - 1].chars().count();
        let target: Vec<u8> = parts[parts.len() - 1]
            .trim_matches(['{', '}'])
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        let target_value = Value { digits: target };

        let nums: Vec<Value> = parts[1..parts.len() - 1]
            .iter()
            .map(|x| {
                let mut vec = vec![0; bit_len];
                x[1..x.len() - 1]
                    .split(',')
                    .map(|y| y.parse::<usize>().unwrap())
                    .for_each(|z| vec[z] = 1);
                Value { digits: vec }
            })
            .collect();

        let mut solver = Solver::new(nums.clone(), target_value.clone());
        result += solver.solve();

        // println!("{:?}", result);
    }

    println!("{}", result);
}
