#[path = "../src/intcode.rs"]
mod intcode;
use intcode::*;

use std::io;

fn testprogram(startprog: Vec<Opcode>, result: Vec<Opcode>) {
    let mut prog: IntCode = startprog.iter().collect();
    let mut hashresult = OpcodeList::new();
    result.iter().enumerate().for_each(|(i, v)| {
        hashresult.insert(i, *v);
    });
    assert_eq!(*prog.run(&mut io::stdin().lock(), &mut io::stdout()), hashresult);
}

#[test]
fn day2_part1_1() {
    testprogram(
    vec![
        1,9,10,3,
        2,3,11,0,
        99,
        30,40,50
    ],
    vec![
        3500,9,10,70,
        2,3,11,0,
        99,
        30,40,50]);
}

#[test]
fn day2_part1_2() {
    testprogram(vec![1,0,0,0,99], vec![2,0,0,0,99])
}

#[test]
fn day2_part1_3() {
    testprogram(vec![2,3,0,3,99], vec![2,3,0,6,99])
}

#[test]
fn day2_part1_4() {
    testprogram(vec![2,4,4,5,99,0], vec![2,4,4,5,99,9801])
}

#[test]
fn day2_part1_5 () {
    testprogram(vec![1,1,1,4,99,5,6,0,99], vec![30,1,1,4,2,5,6,0,99])
}