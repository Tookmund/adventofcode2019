use std::convert::TryFrom;
use std::iter::FromIterator;
use std::collections::HashMap;

use std::io;

pub type Opcode = i64;
pub type OpcodeList = HashMap<usize, Opcode>;

#[derive(Debug)]
enum OP {
    ADD = 1,
    MULTIPLY,
    INPUT,
    OUTPUT,
    JUMPTRUE,
    JUMPFALSE,
    LESS,
    EQUAL,
    RELATIVE,
    EXIT = 99
}

#[derive(Debug)]
enum MODE {
    POSITION = 0,
    IMMEDIATE,
    RELATIVE 
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
            x if x == OP::JUMPTRUE as Opcode => Ok(OP::JUMPTRUE),
            x if x == OP::JUMPFALSE as Opcode => Ok(OP::JUMPFALSE),
            x if x == OP::LESS as Opcode => Ok(OP::LESS),
            x if x == OP::EQUAL as Opcode => Ok(OP::EQUAL),
            x if x == OP::RELATIVE as Opcode => Ok(OP::RELATIVE),
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
            x if x == MODE::RELATIVE as Opcode => Ok(MODE::RELATIVE),
            _ => Err(())
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntCode {
    mem: OpcodeList,
    ip: usize,
    rel: Opcode
}

impl IntCode {
    pub fn new() -> Self {
        IntCode {
            mem: OpcodeList::new(),
            ip: 0,
            rel: 0
        }
    }

    pub fn run<R: io::BufRead, W: io::Write>(&mut self, input: &mut R, output: &mut W) -> &OpcodeList {
        while self.ip < self.mem.len() {
            let op = self.ip2op();
            match op {
                OP::ADD => self.addop(),
                OP::MULTIPLY => self.multop(),
                OP::INPUT => self.inputop(input),
                OP::OUTPUT => self.outputop(output),
                OP::JUMPTRUE => self.jumpop(true),
                OP::JUMPFALSE => self.jumpop(false),
                OP::LESS => self.lessop(),
                OP::EQUAL => self.equalop(),
                OP::RELATIVE => self.relop(),
                OP::EXIT => break
            };
            match op {
                OP::JUMPTRUE | OP::JUMPFALSE => (),
                _ => self.ip += 1
            }
        }
        &self.mem
    }

    fn getmem(&self, loc: usize) -> Opcode {
        match self.mem.get(&loc) {
            Some(v) => *v,
            None => 0
        }
    }

    fn setmem(&mut self, loc: usize, val: Opcode) {
        self.mem.insert(loc, val);
    }

    fn ip2op(&self) -> OP {
        // Last two digits are the opcode
        let op = self.getmem(self.ip) % 100;
        match OP::try_from(op) {
            Ok(v) => v,
            Err(_) => panic!("{}: Invalid opcode {}!", self.ip, op)
        }
    }

    fn ip2opmodes(&self) -> Opcode {
        // Rest of the digits are arg modes
        self.getmem(self.ip) / 100
    }

    fn getmode(&self, opmodes: Opcode, i: Opcode) -> MODE {
        let mut mode = opmodes % (10*i);
        if i > 1 {
            mode /= 10 * (i-1);
        }
    
        match MODE::try_from(mode) {
            Ok(v) => v,
            Err(_) => panic!("{}: Invalid mode {}!", self.ip, mode)
        }
    }

    fn getarg(&mut self, mode: MODE) -> Opcode {
        self.ip += 1;
        match mode {
            MODE::IMMEDIATE => self.getmem(self.ip),
            MODE::POSITION => {
                let pos = self.getpos();
                self.getmem(pos)
            },
            MODE::RELATIVE => {
                let pos = self.rel + self.getmem(self.ip);
                match usize::try_from(pos) {
                    Ok(v) => self.getmem(v),
                    Err(_) => panic!("{}: Invalid position {}!", self.ip, pos)
                }
            }
        }
    }

    fn getpos(&self) -> usize {
        let pos = self.getmem(self.ip);
        match usize::try_from(pos) {
            Ok(v) => v,
            Err(_) => panic!("{}: Invalid position {}!", self.ip, pos)
        }
    }

    fn setpos(&mut self, result: Opcode) {
        self.ip += 1;
        let pos = self.getpos();
        self.setmem(pos, result);
    }

    fn addop(&mut self) {
        let opmodes = self.ip2opmodes();
        let add1 = self.getarg(self.getmode(opmodes, 1));
        let add2 = self.getarg(self.getmode(opmodes, 2));
        self.setpos(add1 + add2);
    }

    fn multop(&mut self) {
        let opmodes = self.ip2opmodes();
        let mult1 = self.getarg(self.getmode(opmodes, 1));
        let mult2 = self.getarg(self.getmode(opmodes, 2));
        self.setpos(mult1 * mult2);
    }

    fn inputop<R: io::BufRead>(&mut self, input: &mut R) {
        let mut input_str = String::new();
        match input.read_line(&mut input_str) {
            Ok(_) => (),
            Err(_) => panic!("{}: Failed to read input!", self.ip),
        };
        let inputopcode: Opcode  = match input_str.trim().parse() {
            Ok(v) => v,
            Err(_) => panic!("{}: Invalid input number {}!", self.ip, input_str)
        };
        self.setpos(inputopcode);
    }

    fn outputop<W: io::Write>(&mut self, output: &mut W) {
        let val = self.getarg(self.getmode(self.ip2opmodes(), 1));
        match writeln!(output, "{}", val) {
            Ok(_) => (),
            Err(_) => panic!("{}: Unable to write to provided output!", self.ip)
        }
    }

    fn jumpop(&mut self, jumptrue: bool) {
        let opmodes = self.ip2opmodes();
        let cond = self.getarg(self.getmode(opmodes, 1));
        let loc = self.getarg(self.getmode(opmodes, 2));
        // Only one is true
        if jumptrue ^ (cond == 0) {
            self.ip = match usize::try_from(loc) {
                Ok(v) => v,
                Err(_) => panic!("{}: Invalid jump location {}!", self.ip, loc)
            };
        }
        else {
            self.ip += 1;
        }
    }

    fn lessop(&mut self) {
        let opmodes = self.ip2opmodes();
        let param1 = self.getarg(self.getmode(opmodes, 1));
        let param2 = self.getarg(self.getmode(opmodes, 2));
        self.setpos(if param1 < param2 { 1 } else { 0 });
    }

    fn equalop(&mut self) {
        let opmodes = self.ip2opmodes();
        let param1 = self.getarg(self.getmode(opmodes, 1));
        let param2 = self.getarg(self.getmode(opmodes, 2));
        self.setpos(if param1 == param2 { 1 } else { 0 });
    }

    fn relop(&mut self) {
        let val = self.getarg(self.getmode(self.ip2opmodes(), 1));
        self.rel += val;
    }
}


impl FromIterator<Opcode> for IntCode {
    fn from_iter<I: IntoIterator<Item=Opcode>>(iter: I) -> Self {
        let mut intcode = IntCode::new();
        iter.into_iter().enumerate().for_each(|(i, v)| {
            intcode.mem.insert(i, v);
        });
        intcode
    }
}

impl<'a> FromIterator<&'a Opcode> for IntCode {
    fn from_iter<I: IntoIterator<Item=&'a Opcode>>(iter: I) -> Self {
        let mut intcode = IntCode::new();
        iter.into_iter().enumerate().for_each(|(i, v)| {
            intcode.mem.insert(i, *v);
        });
        intcode
    }
}