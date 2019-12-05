use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let inputs = contents.split("\n");
    let mut feul = 0.0_f64;

    for input in inputs {
        if input != "" {
            feul += (input.parse::<f64>().unwrap()/3_f64).floor()-2_f64;
        }
    }

    println!("{}",feul);
    Ok(())
}
