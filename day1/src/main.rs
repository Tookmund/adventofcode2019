use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut total: i32 = 0;
    for line in io::stdin().lock().lines() {
        let massint = line?.parse::<i32>().unwrap();
        total += getfuel(massint);
    }
    println!("Total Fuel Required: {}", total);
    Ok(())
}

fn getfuel(mass: i32) -> i32 {
    (mass / 3) - 2
}
