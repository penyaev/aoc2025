use aoc_2025::utils::{input_file, read_lines};
use std::cmp::{max, min};
use std::collections::BTreeMap;

struct Loop {
    edges_x: BTreeMap<u64, (u64, u64)>,
    edges_y: BTreeMap<u64, (u64, u64)>,
    points: Vec<(u64, u64)>,
}

impl Loop {
    fn new() -> Self {
        Self {
            edges_x: BTreeMap::new(),
            edges_y: BTreeMap::new(),
            points: Vec::new(),
        }
    }

    fn add_edge(&mut self, from: (u64, u64), to: (u64, u64)) {
        if from.0 == to.0 {
            self.edges_x.insert(from.0, (from.1, to.1));
        } else if from.1 == to.1 {
            self.edges_y.insert(from.1, (from.0, to.0));
        } else {
            panic!("unexpected edge");
        }
    }

    fn add_point(&mut self, point: (u64, u64)) {
        if self.points.len() > 0 {
            self.add_edge(*self.points.last().unwrap(), point);
        }
        self.points.push(point);
    }

    fn done(&mut self) {
        self.add_edge(*self.points.last().unwrap(), *self.points.first().unwrap());
    }

    /// returns true if the rectangle formed by p1 and p2 intersects with any vertical edge
    fn rect_intersects_vertical(&self, p1: (u64, u64), p2: (u64, u64)) -> bool {
        if p1.0 == p2.0 {
            return false;
        }

        self.edges_x
            .range(min(p1.0, p2.0) + 1..max(p1.0, p2.0))
            .find(|(_, (y1, y2))| {
                let outside = (min(*y1, *y2) < min(p1.1, p2.1) && max(*y1, *y2) <= min(p1.1, p2.1))
                    || (min(*y1, *y2) >= max(p1.1, p2.1) && max(*y1, *y2) > max(p1.1, p2.1));

                !outside
            })
            .is_some()
    }

    /// returns true if the rectangle formed by p1 and p2 intersects with any horizontal edge
    fn rect_intersects_horizontal(&self, p1: (u64, u64), p2: (u64, u64)) -> bool {
        if p1.1 == p2.1 {
            return false;
        }

        self.edges_y
            .range(min(p1.1, p2.1) + 1..max(p1.1, p2.1))
            .find(|(_, (x1, x2))| {
                let outside = (min(*x1, *x2) < min(p1.0, p2.0) && max(*x1, *x2) <= min(p1.0, p2.0))
                    || (min(*x1, *x2) >= max(p1.0, p2.0) && max(*x1, *x2) > max(p1.0, p2.0));

                !outside
            })
            .is_some()
    }

    /// returns true if the rectangle formed by p1 and p2 intersects with any part of the loop
    fn rect_intersects(&self, p1: (u64, u64), p2: (u64, u64)) -> bool {
        self.rect_intersects_horizontal(p1, p2)
            || self.rect_intersects_vertical(p1, p2)
    }

    /// returns all pairs of points that form rectangles that do not intersect the loop
    fn not_intersecting_rects(&self) -> Vec<((u64, u64), (u64, u64))> {
        let mut result = Vec::new();

        for (i, p1) in self.points.iter().enumerate() {
            for p2 in self.points.iter().skip(i + 1) {
                let ok = !self.rect_intersects(*p1, *p2);
                if ok {
                    result.push((*p1, *p2));
                }
            }
        }

        result
    }

    /// returns true if the rectangle formed by p1 and p2 is inside the loop
    fn rect_inside(&self, p1: (u64, u64), p2: (u64, u64)) -> bool {
        if p1.0 == p2.0 || p1.1 == p2.1 {
            return true;
        }

        let test = (min(p1.0, p2.0)+1, min(p1.1, p2.1)+1);

        self.edges_x.range(0..test.0).filter(|(_, (y1, y2))| min(*y1, *y2) <= test.1 && max(*y1, *y2) > test.1).count() % 2 == 1
    }
}

fn main() {
    let input = read_lines(input_file(9, false)).expect("failed to read input");

    let mut l = Loop::new();
    for line_result in input {
        let line = line_result.expect("failed to parse line");

        let nums: Vec<u64> = line
            .split(',')
            .map(|x| x.parse().expect("failed to parse number"))
            .collect();

        assert_eq!(nums.len(), 2);
        l.add_point((nums[0], nums[1]));
    }
    l.done();

    let mut candidates = l.not_intersecting_rects();
    candidates.sort_by(|a, b| {
        let area_a = (a.0 .0.abs_diff(a.1 .0) + 1) * (a.0 .1.abs_diff(a.1 .1) + 1);
        let area_b = (b.0 .0.abs_diff(b.1 .0) + 1) * (b.0 .1.abs_diff(b.1 .1) + 1);
        area_b.cmp(&area_a)
    });

    let best = candidates.iter().find(|(p1, p2)| l.rect_inside(*p1, *p2)).unwrap();
    let best_area = (best.0 .0.abs_diff(best.1 .0) + 1) * (best.0 .1.abs_diff(best.1 .1) + 1);
    println!("{:?}", best_area);
}
