use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("filename arugment");
    let input = File::open(filename).expect("file not found");
    let reader = BufReader::new(input);

    // Build a list of unique co-ordinates for each represented wire
    let mut wires: Vec<Vec<(isize, isize)>> = Vec::new();
    for line in reader.lines() {
        let mut wire = Vec::new();
        let mut position = (0, 0);
        for path in line.expect("read wire").split(",").map(to_path) {
            for point in path.all_points(&mut position) {
                wire.push(point);
            }
        }
        wires.push(wire);
    }

    // Check for co-ordinates that appear for all wires
    let mut contact_points: HashSet<(isize, isize)> = HashSet::new();
    for points in wires.clone() {
        let set: HashSet<(isize, isize)> = HashSet::from_iter(points);
        if contact_points.is_empty() {
            contact_points = set;
        } else {
            contact_points = contact_points.intersection(&set).cloned().collect();
        }
    }

    // For each contact point, count the total number of steps for all wires
    // Since we are finding the minimal number of steps, we must consider every
    // contact point rather than just picking the first we encounter on a wire
    // due to weird edge cases
    let mut closest = (usize::max_value(), (0, 0));
    for point in contact_points {
        let mut total_steps = 0;
        for wire in wires.clone() {
            let steps = wire.iter().position(|p| *p == point).unwrap() + 1;
            total_steps += steps;
        }
        println!("Contact point {:?} is {:?} steps away", point, total_steps);
        if total_steps < closest.0 {
            closest.0 = total_steps;
            closest.1 = point
        }
    }
    println!("Done");
    println!("Contact point {:?} is {:?} steps away", closest.1, closest.0);
}

#[derive(Debug)]
struct Path {
    direction: char,
    length: isize
}

impl Path {
    fn all_points(&self, position: &mut (isize, isize)) -> Vec<(isize, isize)> {
        let mut nodes = Vec::new();
        for _ in 0..self.length {
            match self.direction {
                'R' => position.0 += 1,
                'L' => position.0 -= 1,
                'U' => position.1 -= 1,
                'D' => position.1 += 1,
                _ => panic!("unknown direction")
            };
            nodes.push(position.clone());
        }
        return nodes;
    }
}

fn to_path(input: &str) -> Path {

    let input = String::from(input);
    let (dir, len) = input.split_at(1);

    return Path { 
        direction: dir.parse().unwrap(), 
        length: len.parse().unwrap()
    };
}
