mod intcode;

use std::io;
use intcode::*;


fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Unable to read program from stdin!");
    let mut prog: IntCode = line.split(",").map(|s| s.trim().parse::<Opcode>().unwrap()).collect();
    prog.run(&mut io::stdin().lock(), &mut io::stdout());
}