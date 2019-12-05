use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let inputs = contents.split("\n");
    let mut fuel = 0.0_f64;

    for input in inputs {
        if input != "" {
            fuel += need_fuel(input.parse::<f64>().unwrap());
        }
    }

    println!("{}",fuel);
    Ok(())
}

fn need_fuel(mass: f64) -> f64 {

    let mut fuel = (mass / 3_f64).floor() - 2_f64;

    if fuel > 0_f64 {
        fuel += need_fuel(fuel)
    } else {
        return 0_f64
    }

    return fuel
}
