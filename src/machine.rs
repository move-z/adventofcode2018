use lazy_static::lazy_static;
use regex::Regex;

use crate::*;

#[derive(PartialEq, Debug)]
pub enum MachineState {
    Active,
    Halted,
}

pub struct Machine<const N: usize> {
    pub registers: Registers<N>,
    pub ip_reg: u8,
    program: Vec<OpCode>,
    pub state: MachineState,
}

impl<const N: usize> Machine<N> {
    pub fn new(ip_reg: u8, program: Vec<OpCode>, registers: Registers<N>) -> Machine<N> {
        Machine {
            registers,
            ip_reg,
            program,
            state: MachineState::Active,
        }
    }

    pub fn curr_ip(&self) -> i32 {
        let ip_reg = &(self.ip_reg as i32);
        self.registers.get(ip_reg)
    }

    pub fn execute_step(&mut self) {
        let ip_reg = &(self.ip_reg as i32);
        let ip = self.registers.get(ip_reg);

        if let Some(instruction) = self.program.get(ip as usize) {
            let mut new_registers = instruction.apply(&self.registers);
            new_registers.set(ip_reg, new_registers.get(ip_reg) + 1);
            self.registers = new_registers;
        } else {
            self.state = MachineState::Halted;
        }
    }

    pub fn run(&mut self) {
        while let MachineState::Active = self.state {
            self.execute_step();
        }
    }

    pub fn parse(input: &[&str]) -> Machine<N> {
        let mut ip_reg = 0;
        let mut program = Vec::new();

        for line in input {
            if line.starts_with('#') {
                ip_reg = line.chars().last().unwrap().to_string().parse().unwrap();
            } else if let Some(op) = OpCode::parse(line) {
                program.push(op);
            }
        }

        Machine::new(ip_reg, program, Registers::default())
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Registers<const N: usize> {
    inner: [i32; N],
}

impl<const N: usize> Registers<N> {
    pub fn new(inner: [i32; N]) -> Registers<N> {
        Registers {
            inner,
        }
    }

    pub fn get(&self, i: &i32) -> i32 {
        *self.inner.get(*i as usize).unwrap_or(&0)
    }

    pub fn set(&mut self, i: &i32, val: i32) {
        let i = *i as usize;
        if i < self.inner.len() {
            self.inner[i] = val;
        }
    }

    fn default() -> Self {
        Registers {
            inner: [0; N],
        }
    }

    pub fn all(&self) -> [i32; N] {
        self.inner
    }
}

lazy_static! {
    static ref OP_RE: Regex = Regex::new(r"([[:alpha:]]+) (\d+) (\d+) (\d+)").unwrap();
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum OpCode {
    AddR(i32, i32, i32),
    AddI(i32, i32, i32),
    MulR(i32, i32, i32),
    MulI(i32, i32, i32),
    BanR(i32, i32, i32),
    BanI(i32, i32, i32),
    BorR(i32, i32, i32),
    BorI(i32, i32, i32),
    SetR(i32, i32, i32),
    SetI(i32, i32, i32),
    GtIr(i32, i32, i32),
    GtRi(i32, i32, i32),
    GtRr(i32, i32, i32),
    EqIr(i32, i32, i32),
    EqRi(i32, i32, i32),
    EqRr(i32, i32, i32),
    Nop,
}

impl OpCode {
    pub fn apply<const T: usize>(&self, input: &Registers<T>) -> Registers<T> {
        let mut res = input.clone();
        match self {
            OpCode::AddR(a, b, c) => res.set(c, res.get(a).overflowing_add(res.get(b)).0),
            OpCode::AddI(a, b, c) => res.set(c, res.get(a).overflowing_add(*b).0),

            OpCode::MulR(a, b, c) => res.set(c, res.get(a).overflowing_mul(res.get(b)).0),
            OpCode::MulI(a, b, c) => res.set(c, res.get(a).overflowing_mul(*b).0),

            OpCode::BanR(a, b, c) => res.set(c, res.get(a) & res.get(b)),
            OpCode::BanI(a, b, c) => res.set(c, res.get(a) & b),

            OpCode::BorR(a, b, c) => res.set(c, res.get(a) | res.get(b)),
            OpCode::BorI(a, b, c) => res.set(c, res.get(a) | b),

            OpCode::SetR(a, _, c) => res.set(c, res.get(a)),
            OpCode::SetI(a, _, c) => res.set(c, *a),

            OpCode::GtIr(a, b, c) => res.set(c, if *a > res.get(b) { 1 } else { 0 }),
            OpCode::GtRi(a, b, c) => res.set(c, if res.get(a) > *b { 1 } else { 0 }),
            OpCode::GtRr(a, b, c) => res.set(c, if res.get(a) > res.get(b) { 1 } else { 0 }),

            OpCode::EqIr(a, b, c) => res.set(c, if *a == res.get(b) { 1 } else { 0 }),
            OpCode::EqRi(a, b, c) => res.set(c, if res.get(a) == *b { 1 } else { 0 }),
            OpCode::EqRr(a, b, c) => res.set(c, if res.get(a) == res.get(b) { 1 } else { 0 }),
            _ => {}
        }

        res
    }

    pub fn get_default(&self) -> OpCode {
        match self {
            OpCode::AddR(_, _, _) => OpCode::AddR(0, 0, 0),
            OpCode::AddI(_, _, _) => OpCode::AddI(0, 0, 0),
            OpCode::MulR(_, _, _) => OpCode::MulR(0, 0, 0),
            OpCode::MulI(_, _, _) => OpCode::MulI(0, 0, 0),
            OpCode::BanR(_, _, _) => OpCode::BanR(0, 0, 0),
            OpCode::BanI(_, _, _) => OpCode::BanI(0, 0, 0),
            OpCode::BorR(_, _, _) => OpCode::BorR(0, 0, 0),
            OpCode::BorI(_, _, _) => OpCode::BorI(0, 0, 0),
            OpCode::SetR(_, _, _) => OpCode::SetR(0, 0, 0),
            OpCode::SetI(_, _, _) => OpCode::SetI(0, 0, 0),
            OpCode::GtIr(_, _, _) => OpCode::GtIr(0, 0, 0),
            OpCode::GtRi(_, _, _) => OpCode::GtRi(0, 0, 0),
            OpCode::GtRr(_, _, _) => OpCode::GtRr(0, 0, 0),
            OpCode::EqIr(_, _, _) => OpCode::EqIr(0, 0, 0),
            OpCode::EqRi(_, _, _) => OpCode::EqRi(0, 0, 0),
            OpCode::EqRr(_, _, _) => OpCode::EqRr(0, 0, 0),
            _ => *self,
        }
    }

    fn parse(input: &str) -> Option<OpCode> {
        if let Some(cap) = OP_RE.captures(input) {
            let o: String = parse_capture(&cap, 1, "o").unwrap();
            let a = parse_capture(&cap, 2, "a").unwrap();
            let b = parse_capture(&cap, 3, "b").unwrap();
            let c = parse_capture(&cap, 4, "c").unwrap();

            let op = match o.as_str() {
                "addr" => OpCode::AddR(a, b, c),
                "addi" => OpCode::AddI(a, b, c),
                "mulr" => OpCode::MulR(a, b, c),
                "muli" => OpCode::MulI(a, b, c),
                "banr" => OpCode::BanR(a, b, c),
                "bani" => OpCode::BanI(a, b, c),
                "borr" => OpCode::BorR(a, b, c),
                "bori" => OpCode::BorI(a, b, c),
                "setr" => OpCode::SetR(a, b, c),
                "seti" => OpCode::SetI(a, b, c),
                "gtir" => OpCode::GtIr(a, b, c),
                "gtri" => OpCode::GtRi(a, b, c),
                "gtrr" => OpCode::GtRr(a, b, c),
                "eqir" => OpCode::EqIr(a, b, c),
                "eqri" => OpCode::EqRi(a, b, c),
                "eqrr" => OpCode::EqRr(a, b, c),
                _ => OpCode::Nop,
            };

            Some(op)
        } else {
            None
        }
    }
}
