use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut total: i32 = 0;
    for line in io::stdin().lock().lines() {
        let massint = line?.parse::<i32>().unwrap();
        let mut newfuel = getfuel(massint);
        while newfuel >= 0 {
            total += newfuel;
            newfuel = getfuel(newfuel);
        }
    }
    println!("Total Fuel Required: {}", total);
    Ok(())
}

fn getfuel(mass: i32) -> i32 {
    (mass / 3) - 2
}