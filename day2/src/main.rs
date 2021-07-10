use std::io;
use std::io::prelude::*;

type Opcode = usize;

fn main() {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let mut prog: Vec<Opcode> = line.split(",").map(|s| s.parse().unwrap()).collect();
    // Replace to restore the values of the running program
    prog[1] = 12;
    prog[2] = 2;
    let result = runprogram(&prog);
    println!("{}", result[0]);
}

fn runprogram(startprog: &Vec<Opcode>) -> Vec<Opcode> {
    let mut prog = startprog.clone();
    let mut op = 0;
    while op <= prog.len() {
        match prog[op] {
            // add
            1 => {
                let op1 = prog[op+1];
                let op2 = prog[op+2];
                let result = prog[op+3];
                prog[result] = prog[op1] + prog[op2];
            },
            // multiply
            2 => {
                let op1 = prog[op+1];
                let op2 = prog[op+2];
                let result = prog[op+3];
                prog[result] = prog[op1] * prog[op2];
            },
            // exit
            99 => break,
            _ => panic!("{} is not a valid opcode!", op)
        }
        op += 4;
    }
    prog
}

#[test]
fn part1_1() {
assert_eq!(runprogram(
    &vec![
        1,9,10,3,
        2,3,11,0,
        99,
        30,40,50
    ]),
    vec![
        3500,9,10,70,
        2,3,11,0,
        99,
        30,40,50]);
}

#[test]
fn part1_2() {
    assert_eq!(runprogram(&vec![1,0,0,0,99]), vec![2,0,0,0,99])
}

#[test]
fn part1_3() {
    assert_eq!(runprogram(&vec![2,3,0,3,99]), vec![2,3,0,6,99])
}

#[test]
fn part1_4() {
    assert_eq!(runprogram(&vec![2,4,4,5,99,0]), vec![2,4,4,5,99,9801])
}

#[test]
fn part1_5 () {
    assert_eq!(runprogram(&vec![1,1,1,4,99,5,6,0,99]), vec![30,1,1,4,2,5,6,0,99])
}
