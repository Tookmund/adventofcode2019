use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut total: i32 = 0;
    for line in io::stdin().lock().lines() {
        let massint = line?.parse::<i32>().unwrap();
        total += rocketfuel(massint);
    }
    println!("Total Fuel Required: {}", total);
    Ok(())
}

// Fuel itself requires fuel because it too has weight
fn rocketfuel(mass: i32) -> i32 {
    let mut result = 0;
    let mut newfuel = getfuel(mass);
    while newfuel >= 0 {
        result += newfuel;
        newfuel = getfuel(newfuel);
    }
    result
}

fn getfuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

#[test]
fn part1() {
    assert_eq!(getfuel(12), 2);
    assert_eq!(getfuel(14), 2);
    assert_eq!(getfuel(1969), 654);
    assert_eq!(getfuel(100756), 33583);
}

#[test]
fn part2() {
    assert_eq!(rocketfuel(14), 2);
    assert_eq!(rocketfuel(1969), 966);
    assert_eq!(rocketfuel(100756), 50346);
}