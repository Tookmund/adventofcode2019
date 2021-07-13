use std::collections::HashMap;
use std::convert::TryFrom;
use std::iter::FromIterator;

use std::io;

pub type Opcode = i64;
pub type OpcodeList = HashMap<usize, Opcode>;

#[derive(Debug)]
enum OP {
    Add = 1,
    Multiply,
    Input,
    Output,
    JumpTrue,
    JumpFalse,
    Less,
    Equal,
    RelativeBase,
    Exit = 99,
}

#[derive(Debug)]
enum MODE {
    Position = 0,
    Immediate,
    Relative,
}

// This needs a macro
impl TryFrom<Opcode> for OP {
    type Error = ();

    fn try_from(val: Opcode) -> Result<Self, Self::Error> {
        match val {
            x if x == OP::Add as Opcode => Ok(OP::Add),
            x if x == OP::Multiply as Opcode => Ok(OP::Multiply),
            x if x == OP::Input as Opcode => Ok(OP::Input),
            x if x == OP::Output as Opcode => Ok(OP::Output),
            x if x == OP::Exit as Opcode => Ok(OP::Exit),
            x if x == OP::JumpTrue as Opcode => Ok(OP::JumpTrue),
            x if x == OP::JumpFalse as Opcode => Ok(OP::JumpFalse),
            x if x == OP::Less as Opcode => Ok(OP::Less),
            x if x == OP::Equal as Opcode => Ok(OP::Equal),
            x if x == OP::RelativeBase as Opcode => Ok(OP::RelativeBase),
            _ => Err(()),
        }
    }
}

impl TryFrom<Opcode> for MODE {
    type Error = ();

    fn try_from(val: Opcode) -> Result<Self, Self::Error> {
        match val {
            x if x == MODE::Position as Opcode => Ok(MODE::Position),
            x if x == MODE::Immediate as Opcode => Ok(MODE::Immediate),
            x if x == MODE::Relative as Opcode => Ok(MODE::Relative),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntCode {
    mem: OpcodeList,
    ip: usize,
    rel: Opcode,
}

impl IntCode {
    pub fn new() -> Self {
        IntCode {
            mem: OpcodeList::new(),
            ip: 0,
            rel: 0,
        }
    }

    pub fn run<R: io::BufRead, W: io::Write>(
        &mut self,
        input: &mut R,
        output: &mut W,
    ) -> &OpcodeList {
        while self.ip < self.mem.len() {
            let op = self.ip2op();
            match op {
                OP::Add => self.addop(),
                OP::Multiply => self.multop(),
                OP::Input => self.inputop(input),
                OP::Output => self.outputop(output),
                OP::JumpTrue => self.jumpop(true),
                OP::JumpFalse => self.jumpop(false),
                OP::Less => self.lessop(),
                OP::Equal => self.equalop(),
                OP::RelativeBase => self.relop(),
                OP::Exit => break,
            };
            match op {
                OP::JumpTrue | OP::JumpFalse => (),
                _ => self.ip += 1,
            }
        }
        &self.mem
    }

    fn getmem(&self, loc: usize) -> Opcode {
        match self.mem.get(&loc) {
            Some(v) => *v,
            None => 0,
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
            Err(_) => panic!("{}: Invalid opcode {}!", self.ip, op),
        }
    }

    fn ip2opmodes(&self) -> Opcode {
        // Rest of the digits are arg modes
        self.getmem(self.ip) / 100
    }

    fn getmode(&self, opmodes: Opcode, i: u32) -> MODE {
        let mode = (opmodes % Opcode::pow(10, i)) / Opcode::pow(10, i - 1);

        match MODE::try_from(mode) {
            Ok(v) => v,
            Err(_) => panic!("{}: Invalid mode {}!", self.ip, mode),
        }
    }

    fn getarg(&mut self, mode: MODE) -> Opcode {
        self.ip += 1;
        match mode {
            MODE::Immediate => self.getmem(self.ip),
            MODE::Position => {
                let pos = self.toposition(self.getmem(self.ip));
                self.getmem(pos)
            }
            MODE::Relative => {
                let relpos = self.rel + self.getmem(self.ip);
                let pos = self.toposition(relpos);
                self.getmem(pos)
            }
        }
    }

    fn toposition(&self, pos: Opcode) -> usize {
        match usize::try_from(pos) {
            Ok(v) => v,
            Err(_) => panic!("{}: Invalid position {}!", self.ip, pos),
        }
    }

    fn setpos(&mut self, result: Opcode, mode: MODE) {
        self.ip += 1;
        let argpos = match mode {
            MODE::Position => self.getmem(self.ip),
            MODE::Relative => self.getmem(self.ip) + self.rel,
            MODE::Immediate => panic!("{}: Immediate mode invalid for set!", self.ip),
        };
        let pos = self.toposition(argpos);
        self.setmem(pos, result);
    }

    fn addop(&mut self) {
        let opmodes = self.ip2opmodes();
        let add1 = self.getarg(self.getmode(opmodes, 1));
        let add2 = self.getarg(self.getmode(opmodes, 2));
        self.setpos(add1 + add2, self.getmode(opmodes, 3));
    }

    fn multop(&mut self) {
        let opmodes = self.ip2opmodes();
        let mult1 = self.getarg(self.getmode(opmodes, 1));
        let mult2 = self.getarg(self.getmode(opmodes, 2));
        self.setpos(mult1 * mult2, self.getmode(opmodes, 3));
    }

    fn inputop<R: io::BufRead>(&mut self, input: &mut R) {
        let mut input_str = String::new();
        match input.read_line(&mut input_str) {
            Ok(_) => (),
            Err(_) => panic!("{}: Failed to read input!", self.ip),
        };
        let inputopcode: Opcode = match input_str.trim().parse() {
            Ok(v) => v,
            Err(_) => panic!("{}: Invalid input number {}!", self.ip, input_str),
        };
        self.setpos(inputopcode, self.getmode(self.ip2opmodes(), 1));
    }

    fn outputop<W: io::Write>(&mut self, output: &mut W) {
        let val = self.getarg(self.getmode(self.ip2opmodes(), 1));
        match writeln!(output, "{}", val) {
            Ok(_) => (),
            Err(_) => panic!("{}: Unable to write to provided output!", self.ip),
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
                Err(_) => panic!("{}: Invalid jump location {}!", self.ip, loc),
            };
        } else {
            self.ip += 1;
        }
    }

    fn lessop(&mut self) {
        let opmodes = self.ip2opmodes();
        let param1 = self.getarg(self.getmode(opmodes, 1));
        let param2 = self.getarg(self.getmode(opmodes, 2));
        self.setpos(
            if param1 < param2 { 1 } else { 0 },
            self.getmode(opmodes, 3),
        );
    }

    fn equalop(&mut self) {
        let opmodes = self.ip2opmodes();
        let param1 = self.getarg(self.getmode(opmodes, 1));
        let param2 = self.getarg(self.getmode(opmodes, 2));
        self.setpos(
            if param1 == param2 { 1 } else { 0 },
            self.getmode(opmodes, 3),
        );
    }

    fn relop(&mut self) {
        let val = self.getarg(self.getmode(self.ip2opmodes(), 1));
        self.rel += val;
    }
}

impl FromIterator<Opcode> for IntCode {
    fn from_iter<I: IntoIterator<Item = Opcode>>(iter: I) -> Self {
        let mut intcode = IntCode::new();
        iter.into_iter().enumerate().for_each(|(i, v)| {
            intcode.mem.insert(i, v);
        });
        intcode
    }
}

impl<'a> FromIterator<&'a Opcode> for IntCode {
    fn from_iter<I: IntoIterator<Item = &'a Opcode>>(iter: I) -> Self {
        let mut intcode = IntCode::new();
        iter.into_iter().enumerate().for_each(|(i, v)| {
            intcode.mem.insert(i, *v);
        });
        intcode
    }
}
