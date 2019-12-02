use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let filename = "input.txt";
    let input = File::open(filename).expect("file not found");
    let reader = BufReader::new(input);

    let mut total = 0_f64;
    for (_, line) in reader.lines().enumerate() {
        let line = line.expect("Can't read line");
        let result = line.parse::<f64>().unwrap();

        let mut remaining_mass = result;
        while remaining_mass > 0_f64 {
            print!("Added {} more mass", remaining_mass);
            let additional_fuel = fuel_required_for_mass(remaining_mass);
            print!(", so packing another {} of fuel\n", additional_fuel);
            total += additional_fuel;
            remaining_mass = additional_fuel;
        }
    }
    println!("Total Fuel Requirement: {}", total);
}

fn fuel_required_for_mass(mass: f64) -> f64 {
    let result = (mass / 3.0_f64).floor();
    let result = result - 2.0_f64;
    return result.max(0_f64);
}
