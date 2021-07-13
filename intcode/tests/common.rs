#[path = "../src/intcode.rs"]
mod intcode;
pub use intcode::*;

pub use std::io;

type TestIO<'a> = &'a [u8];

pub fn testprogram_io(
    startprog: Vec<Opcode>,
    expectedresult: Option<Vec<Opcode>>,
    input: Option<TestIO>,
    expectedout: Option<TestIO>,
) {
    let mut prog: IntCode = startprog.iter().collect();
    let mut output: Vec<u8> = Vec::new();

    let mut bufinput = io::BufReader::new(input.unwrap_or(b" "));
    let result = prog.run(&mut bufinput, &mut output);

    match expectedresult {
        Some(expected) => {
            let mut hashexpected = OpcodeList::new();
            expected.iter().enumerate().for_each(|(i, v)| {
                hashexpected.insert(i, *v);
            });
            assert_eq!(*result, hashexpected);
        }
        None => (),
    }

    match expectedout {
        Some(v) => assert_eq!(*output, *v),
        None => println!("{}", String::from_utf8_lossy(&output)),
    };
}

#[allow(dead_code)]
pub fn testprogram(startprog: Vec<Opcode>, result: Vec<Opcode>) {
    testprogram_io(startprog, Some(result), None, None);
}
