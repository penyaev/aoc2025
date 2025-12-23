use aoc_2025::utils::{input_file, read_lines};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct Value {
    digits: Vec<usize>,
}

impl Value {
    fn new(len: usize) -> Self {
        Self {
            digits: vec![0; len],
        }
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            digits: self
                .digits
                .iter()
                .zip(other.digits.iter())
                .map(|(x, y)| x + y)
                .collect(),
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Self {
            digits: self
                .digits
                .iter()
                .zip(other.digits.iter())
                .map(|(x, y)| x - y)
                .collect(),
        }
    }

    fn div(&self, by: usize) -> Self {
        Self {
            digits: self.digits.iter().map(|x| x / by).collect(),
        }
    }

    fn xor(&self, other: &Self) -> Self {
        Self {
            digits: self
                .digits
                .iter()
                .zip(other.digits.iter())
                .map(|(x, y)| x ^ y)
                .collect(),
        }
    }

    fn parity_mask(&self) -> Self {
        Self {
            digits: self.digits.iter().map(|x| x % 2).collect(),
        }
    }

    fn zero(&self) -> bool {
        self.digits.iter().all(|x| *x == 0)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.digits.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""))
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

struct CombinationsIter<T> {
    vec: Vec<T>,
    pos: usize,
}

impl<T> CombinationsIter<T> {
    fn new(vec: Vec<T>) -> Self {
        Self { vec, pos: 0 }
    }
}

impl<T: Clone> Iterator for CombinationsIter<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let max = 1 << self.vec.len();
        self.pos += 1;
        if self.pos > max {
            return None;
        }

        Some(
            self.vec
                .iter()
                .enumerate()
                .filter_map(|(i, x)| {
                    if self.pos & (1 << i) != 0 {
                        Some(x.clone())
                    } else {
                        None
                    }
                })
                .collect(),
        )
    }
}

struct Solver {
    nums: Vec<Value>,
    target: Value,
    cache: HashMap<Value, Option<usize>>,
}

impl Solver {
    fn new(nums: Vec<Value>, target: Value) -> Self {
        Self { nums, target, cache: HashMap::new() }
    }

    fn solve_for(&mut self, target: &Value) -> Option<usize> {
        if target.zero() {
            return Some(0);
        }
        if self.cache.contains_key(target) {
            return self.cache.get(target).unwrap().clone();
        }

        let iter = CombinationsIter::new(self.nums.clone());
        let parity_mask = target.parity_mask();

        let mut best: Option<usize> = None;
        for nums in iter {
            let matching_parity = nums
                .iter()
                .fold(Value::new(target.digits.len()), |acc, x| acc.xor(x))
                == parity_mask;
            if !matching_parity {
                continue;
            }

            let sum = nums
                .iter()
                .fold(Value::new(target.digits.len()), |acc, x| acc.add(x));

            if sum > *target {
                continue;
            }
            let new_target = target.sub(&sum).div(2);
            let x = self.solve_for(&new_target);
            if x.is_none() {
                continue;
            }
            let x_candidate = x.unwrap() * 2 + nums.len();
            if let Some(best_value) = best {
                if x_candidate < best_value {
                    best = Some(x_candidate);
                }
            } else {
                best = Some(x_candidate);
            }
        }

        // println!("best for {} is {:?}", target, best);
        self.cache.insert(target.clone(), best);
        best
    }

    fn solve(&mut self) -> usize {
        self.solve_for(&self.target.clone()).unwrap()
    }
}

fn main() {
    // solution idea courtesy of:
    // https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
    let input = read_lines(input_file(10, false)).expect("failed to read input");

    let mut result = 0;
    for line_result in input {
        let line = line_result.expect("failed to parse line");

        let parts: Vec<&str> = line.split(' ').collect();

        assert!(parts.len() >= 3);

        let bit_len = parts[0][1..parts[0].len() - 1].chars().count();
        let target: Vec<usize> = parts[parts.len() - 1]
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
