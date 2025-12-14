use aoc_2025::utils::{input_file, read_lines};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

struct Point3D {
    x: u64,
    y: u64,
    z: u64,
}

impl Display for Point3D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

struct JunctionBox {
    location: Point3D,
    circuit_id: u64,
}

#[derive(Debug)]
struct Distance {
    i: usize,
    j: usize,
    distance: f64,
}

fn distance3d(p1: &Point3D, p2: &Point3D) -> f64 {
    let dx = (p1.x as i64 - p2.x as i64) as f64;
    let dy = (p1.y as i64 - p2.y as i64) as f64;
    let dz = (p1.z as i64 - p2.z as i64) as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

impl PartialEq for Distance {
    fn eq(&self, other: &Self) -> bool {
        (self.distance - other.distance).abs() < f64::EPSILON
    }
}

impl Eq for Distance {}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance
            .partial_cmp(&other.distance)
            .unwrap_or(Ordering::Equal)
    }
}

fn main() {
    let input = read_lines(input_file(8, false)).expect("failed to read input");

    let mut boxes: Vec<JunctionBox> = Vec::new();
    let mut distances: Vec<Distance> = Vec::new();
    let mut circuits: HashMap<u64, Vec<usize>> = HashMap::new();

    for line_result in input {
        let line = line_result.unwrap();

        let parts: Vec<u64> = line.split(',').map(|x| x.parse().unwrap()).collect();
        assert_eq!(parts.len(), 3);
        let new_point = Point3D {
            x: parts[0],
            y: parts[1],
            z: parts[2],
        };
        for (i, b) in boxes.iter().enumerate() {
            distances.push(Distance {
                i,
                j: boxes.len(),
                distance: distance3d(&b.location, &new_point),
            });
        }
        let circuit_id = boxes.len() as u64;
        circuits.insert(circuit_id, vec![boxes.len()]);
        boxes.push(JunctionBox {
            location: new_point,
            circuit_id,
        });
    }
    distances.sort();

    let mut result = 0u64;
    for distance in distances.iter() {
        if boxes[distance.i].circuit_id == boxes[distance.j].circuit_id {
            continue;
        }

        let donor_circuit_id = boxes[distance.j].circuit_id;
        let boxes_to_merge = circuits.get(&donor_circuit_id).unwrap().clone();
        boxes_to_merge.iter().for_each(|&box_index| {
            boxes[box_index].circuit_id = boxes[distance.i].circuit_id;
        });
        circuits.entry(boxes[distance.i].circuit_id).and_modify(|x| x.extend(boxes_to_merge));
        circuits.remove(&donor_circuit_id);

        if circuits.len() == 1 {
            result = boxes[distance.i].location.x * boxes[distance.j].location.x;
            break;
        }
    }

    println!("{}", result);
}
