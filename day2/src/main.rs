use std::env;
use std::io;
use std::io::prelude::*;

type Opcode = usize;

const PART2: Opcode = 19690720;
// vector is length 157, so can't be larger than 156
const MAXSIZE: Opcode = 156;

fn main() {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let mut prog: Vec<Opcode> = line.split(",").map(|s| s.parse().unwrap()).collect();
    // Replace to restore the values of the running program
    // 12 and 2 for part 1
    // No arguments for part 2
    let count = env::args().count();
    if count == 1 {
        for one in 0..MAXSIZE {
            for two in 0..MAXSIZE {
                prog[1] = one;
                prog[2] = two;
                let result = runprogram(&prog);
                if result[0] == PART2 {
                    println!("{} and {} = {}", prog[1], prog[2], PART2);
                    println!("100 * noun ({}) + verb ({}) = {}", prog[1], prog[2], 100 * prog[1] + prog[2]);
                    return;
                }
            }
        }
    }
    else if count < 3 {
        let name = env::args().next().unwrap();
        println!("{} <one> <two> \nor just\n{}\nto find arguments matching {}", name, name, PART2);
        return;
    }
    let mut args = env::args();
    args.next();
    for i in 1..3 {
        prog[i] = args.next().unwrap().parse().unwrap();
    }
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
