mod intcode;

use std::io;
use std::io::prelude::*;
use intcode::*;


fn main() {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let prog: IntCode = line.split(",").map(|s| s.parse::<Opcode>().unwrap()).collect();
    println!("{:?}", prog);
}