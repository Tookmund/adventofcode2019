use std::convert::TryFrom;
use std::iter::FromIterator;

pub type Opcode = i32;
pub type OpcodeList = Vec<Opcode>;

#[derive(Debug, Clone)]
pub struct IntCode {
    mem: OpcodeList,
    ip: usize
}

#[derive(Debug)]
enum OP {
    ADD = 1,
    MULTIPLY,
    INPUT,
    OUTPUT,
    EXIT = 99
}

#[derive(Debug)]
enum MODE {
    POSITION = 0,
    IMMEDIATE 
}


// This needs a macro
impl TryFrom<Opcode> for OP {
    type Error = ();

    fn try_from(val: Opcode) -> Result<Self, Self::Error> {
        match val {
            x if x == OP::ADD as Opcode => Ok(OP::ADD),
            x if x == OP::MULTIPLY as Opcode => Ok(OP::MULTIPLY),
            x if x == OP::INPUT as Opcode => Ok(OP::INPUT),
            x if x == OP::OUTPUT as Opcode => Ok(OP::OUTPUT),
            x if x == OP::EXIT as Opcode => Ok(OP::EXIT),
            _ => Err(())
        }
    }
}

impl TryFrom<Opcode> for MODE {
    type Error = ();

    fn try_from(val: Opcode) -> Result<Self, Self::Error> {
        match val {
            x if x == MODE::POSITION as Opcode => Ok(MODE::POSITION),
            x if x == MODE::IMMEDIATE as Opcode => Ok(MODE::IMMEDIATE),
            _ => Err(())
        }
    }
}

impl IntCode {
    pub fn new() -> Self {
        IntCode {
            mem: OpcodeList::new(),
            ip: 0
        }
    }

    pub fn run(&mut self) -> &OpcodeList {
        while self.ip < self.mem.len() {
            let op = self.ip2op();
            println!("Opcode: {:?}", op);
            match op {
                OP::ADD => self.addop(),
                OP::MULTIPLY => self.multop(),
                OP::INPUT => self.inputop(),
                OP::OUTPUT => self.outputop(),
                OP::EXIT => break
            }
            self.ip += 1;
        }
        &self.mem
    }

    fn ip2op(&self) -> OP {
        // Last two digits are the opcode
        let op = self.mem[self.ip] % 100;
        match OP::try_from(op) {
            Ok(v) => v,
            Err(_) => panic!("{} is not a valid opcode!", op)
        }
    }

    fn ip2opmodes(&self) -> Opcode {
        // Rest of the digits are arg modes
        self.mem[self.ip] / 100
    }


    fn getarg(&mut self, mode: MODE) -> Opcode {
        self.ip += 1;
        match mode {
            MODE::IMMEDIATE => self.mem[self.ip],
            MODE::POSITION => {
                let pos = self.getpos();
                self.mem[pos]
            }
        }
    }

    fn getpos(&self) -> usize {
        let pos = self.mem[self.ip];
        match usize::try_from(pos) {
            Ok(v) => v,
            Err(_) => panic!("{} is an invalid position!", pos)
        }
    }

    fn setpos(&mut self, result: Opcode) {
        self.ip += 1;
        let pos = self.getpos();
        self.mem[pos] = result;
    }

    fn addop(&mut self) {
        let opmodes = self.ip2opmodes();
        let add1 = self.getarg(getmode(opmodes, 1));
        let add2 = self.getarg(getmode(opmodes, 2));
        self.setpos(add1 + add2);
    }

    fn multop(&mut self) {
        let opmodes = self.ip2opmodes();
        let mult1 = self.getarg(getmode(opmodes, 1));
        let mult2 = self.getarg(getmode(opmodes, 2));
        self.setpos(mult1 * mult2);
    }

    fn inputop(&mut self) {
        // TODO
    }

    fn outputop(&mut self) {
        let val = self.getarg(getmode(self.ip2opmodes(), 1));
        println!("{}", val);
    }
}

fn getmode(opmodes: Opcode, i: Opcode) -> MODE {
    let mut mode = opmodes % (10*i);
    if i > 1 {
        mode /= 10 * (i-1);
    }

    match MODE::try_from(mode) {
        Ok(v) => v,
        Err(_) => panic!("{} is not a valid mode!", mode)
    }
}

impl FromIterator<Opcode> for IntCode {
    fn from_iter<I: IntoIterator<Item=Opcode>>(iter: I) -> Self {
        let mut intcode = IntCode::new();
        for i in iter {
            intcode.mem.push(i);
        }
        intcode
    }
}

impl<'a> FromIterator<&'a Opcode> for IntCode {
    fn from_iter<I: IntoIterator<Item=&'a Opcode>>(iter: I) -> Self {
        let mut intcode = IntCode::new();
        for &i in iter {
            intcode.mem.push(i);
        }
        intcode
    }
}

#[cfg(test)]
mod day2tests {
    use crate::intcode::*;

    fn testprogram(startprog: OpcodeList, result: OpcodeList) {
        let mut prog: IntCode = startprog.iter().collect();
        assert_eq!(*prog.run(), result);
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
}