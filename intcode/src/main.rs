mod intcode;
use intcode::*;

use std::io;
use std::io::prelude::*;


fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    let mut input = stdin.lock();
    input.read_line(&mut line).expect("Unable to read program from stdin!");
    let mut prog: IntCode = line.split(",").map(|s| s.trim().parse::<Opcode>().unwrap()).collect();
    prog.run(&mut input, &mut io::stdout());
}