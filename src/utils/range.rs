use std::cmp::{max, min};
use std::collections::BTreeMap;

pub struct RangeSet<T> {
    ranges: BTreeMap<T, T>,
}

impl<T: Ord + Copy> RangeSet<T> {
    pub fn new() -> Self {
        Self { ranges: BTreeMap::new() }
    }

    pub fn insert(&mut self, mut from: T, mut to: T) {
        if let Some((&predecessor_from, &predecessor_to)) = self.ranges.range(..=from).next_back() { // rightmost interval with its left boundary <= value
            if predecessor_to >= from {
                from = min(predecessor_from, from);
                to = max(predecessor_to, to);

                self.ranges.remove(&predecessor_from);
            }
        }

        loop {
            let next = self.ranges.range(from..).next();
            match next {
                Some((&next_from, &next_to)) => {
                    if next_from > to {
                        break;
                    }
                    to = max(next_to, to);
                    self.ranges.remove(&next_from);
                }
                None => {
                    break;
                }
            }
        }

        self.ranges.insert(from, to);
    }

    pub fn contains(&self, value: T) -> bool {
        if let Some((_, to)) = self.ranges.range(..=&value).next_back() { // rightmost interval with its left boundary <= value
            value < *to
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.ranges.len()
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (&T, &T)> {
        self.ranges.iter()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_non_overlapping() {
        let mut set = RangeSet::new();
        set.insert(1, 5);
        set.insert(10, 15);

        assert_eq!(set.len(), 2);
        let ranges: Vec<_> = set.enumerate().collect();
        assert_eq!(ranges, vec![(&1, &5), (&10, &15)]);

        assert!(set.contains(1));
        assert!(set.contains(4));
        assert!(!set.contains(5)); // Exclusive upper bound
        assert!(!set.contains(9));
        assert!(set.contains(10));
        assert!(set.contains(14));
        assert!(!set.contains(15));
    }

    #[test]
    fn test_insert_overlapping_merge() {
        let mut set = RangeSet::new();
        set.insert(1, 5);
        set.insert(4, 8); // Overlaps with 1-5

        assert_eq!(set.len(), 1);
        let ranges: Vec<_> = set.enumerate().collect();
        assert_eq!(ranges, vec![(&1, &8)]);

        assert!(set.contains(1));
        assert!(set.contains(4));
        assert!(set.contains(7));
        assert!(!set.contains(8));

        set.insert(2, 3); // Inside existing
        assert_eq!(set.len(), 1);
        let ranges: Vec<_> = set.enumerate().collect();
        assert_eq!(ranges, vec![(&1, &8)]);
        assert!(set.contains(2));
    }

    #[test]
    fn test_insert_enveloping() {
        let mut set = RangeSet::new();
        set.insert(5, 10);
        set.insert(1, 15); // Envelops 5-10

        assert_eq!(set.len(), 1);
        let ranges: Vec<_> = set.enumerate().collect();
        assert_eq!(ranges, vec![(&1, &15)]);

        assert!(set.contains(1));
        assert!(set.contains(5));
        assert!(set.contains(14));
        assert!(!set.contains(15));
    }

    #[test]
    fn test_insert_bridging() {
        let mut set = RangeSet::new();
        set.insert(1, 5);
        set.insert(10, 15);
        set.insert(4, 11); // Bridges 1-5 and 10-15

        assert_eq!(set.len(), 1);
        let ranges: Vec<_> = set.enumerate().collect();
        assert_eq!(ranges, vec![(&1, &15)]);

        assert!(set.contains(1));
        assert!(set.contains(5));
        assert!(set.contains(9));
        assert!(set.contains(14));
        assert!(!set.contains(15));
    }

    #[test]
    fn test_insert_touching() {
        let mut set = RangeSet::new();
        set.insert(1, 5);
        set.insert(5, 10);
        set.insert(15, 20);
        set.insert(10, 15);

        assert_eq!(set.len(), 1);
        let ranges: Vec<_> = set.enumerate().collect();
        assert_eq!(ranges, vec![(&1, &20)]);
    }
}