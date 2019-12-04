use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("filename arugment");
    let input = File::open(filename).expect("file not found");
    let reader = BufReader::new(input);

    // Build a list of unique co-ordinates for each represented wire
    let mut wires: Vec<HashSet<(isize, isize)>> = Vec::new();
    for line in reader.lines() {
        let mut wire = HashSet::new();
        let mut position = (0, 0);
        for path in line.expect("read wire").split(",").map(to_path) {
            for point in path.all_points(&mut position) {
                wire.insert(point);
            }
        }
        wires.push(wire);
    }

    // Check for co-ordinates that appear for all wires
    let mut contact_points: HashSet<_> = wires.pop().unwrap();
    while !wires.is_empty() {
        let next_contact_points: HashSet<_> = wires.pop().unwrap();
        contact_points = contact_points.intersection(&next_contact_points).cloned().collect();
    }
    println!("contact points {:?}", contact_points);

    // Find the point closest to 0, 0
    // ideally we should maintain a heap whilst finding the intersection but meh
    let mut closest = (isize::max_value(), (0, 0));
    for point in contact_points.iter() {
        let distance = point.0.abs() + point.1.abs(); // manhattan distance
        if distance < closest.0 {
            closest.0 = distance;
            closest.1 = *point;
        }
        // we can ignore the fact that there may exist multiple contact points the same distance away
    }
    println!("point: {:?}, distance: {}", closest.1, closest.0);
}

#[derive(Debug)]
struct Path {
    direction: char,
    length: isize
}

impl Path {
    fn all_points(&self, position: &mut (isize, isize)) -> HashSet<(isize, isize)> {
        let mut nodes = HashSet::new();
        for _ in 0..self.length {
            match self.direction {
                'R' => position.0 += 1,
                'L' => position.0 -= 1,
                'U' => position.1 -= 1,
                'D' => position.1 += 1,
                _ => panic!("unknown direction")
            };
            nodes.insert(position.clone());
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
