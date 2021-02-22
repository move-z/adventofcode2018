use adventofcode2018::*;

use regex::Regex;

use crate::MachineState::{Active, Halted};
use lazy_static::lazy_static;

fn first(input: &[&str]) -> usize {
    let mut machine = Machine::parse(input);
    machine.run();
    machine.registers.inner.0 as usize
}

fn second(_input: &[&str]) -> usize {
    unimplemented!()
}

#[derive(Clone)]
struct Registers {
    inner: (i32, i32, i32, i32, i32, i32),
}

impl Registers {
    fn new() -> Self {
        Registers {
            inner: (0, 0, 0, 0, 0, 0),
        }
    }

    fn get(&self, i: &i32) -> i32 {
        match i {
            0 => self.inner.0,
            1 => self.inner.1,
            2 => self.inner.2,
            3 => self.inner.3,
            4 => self.inner.4,
            5 => self.inner.5,
            _ => 0,
        }
    }

    fn set(&self, i: &i32, val: i32) -> Self {
        let mut res = Registers {
            inner: self.inner.clone(),
        };
        match i {
            0 => res.inner.0 = val,
            1 => res.inner.1 = val,
            2 => res.inner.2 = val,
            3 => res.inner.3 = val,
            4 => res.inner.4 = val,
            5 => res.inner.5 = val,
            _ => {}
        };
        res
    }
}

enum OpCode {
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
    GtIR(i32, i32, i32),
    GtRI(i32, i32, i32),
    GtRR(i32, i32, i32),
    EqIR(i32, i32, i32),
    EqRI(i32, i32, i32),
    EqRR(i32, i32, i32),
    Nop,
}

lazy_static! {
    static ref OP_RE: Regex = Regex::new(r"([[:alpha:]]+) (\d+) (\d+) (\d+)").unwrap();
}

impl OpCode {
    fn apply(&self, input: &Registers) -> Registers {
        match self {
            OpCode::AddR(a, b, c) => input.set(c, input.get(a) + input.get(b)),
            OpCode::AddI(a, b, c) => input.set(c, input.get(a) + b),

            OpCode::MulR(a, b, c) => input.set(c, input.get(a) * input.get(b)),
            OpCode::MulI(a, b, c) => input.set(c, input.get(a) * b),

            OpCode::BanR(a, b, c) => input.set(c, input.get(a) & input.get(b)),
            OpCode::BanI(a, b, c) => input.set(c, input.get(a) & b),

            OpCode::BorR(a, b, c) => input.set(c, input.get(a) | input.get(b)),
            OpCode::BorI(a, b, c) => input.set(c, input.get(a) | b),

            OpCode::SetR(a, _, c) => input.set(c, input.get(a)),
            OpCode::SetI(a, _, c) => input.set(c, *a),

            OpCode::GtIR(a, b, c) => input.set(c, if *a > input.get(b) { 1 } else { 0 }),
            OpCode::GtRI(a, b, c) => input.set(c, if input.get(a) > *b { 1 } else { 0 }),
            OpCode::GtRR(a, b, c) => input.set(c, if input.get(a) > input.get(b) { 1 } else { 0 }),

            OpCode::EqIR(a, b, c) => input.set(c, if *a == input.get(b) { 1 } else { 0 }),
            OpCode::EqRI(a, b, c) => input.set(c, if input.get(a) == *b { 1 } else { 0 }),
            OpCode::EqRR(a, b, c) => input.set(c, if input.get(a) == input.get(b) { 1 } else { 0 }),

            _ => input.clone(),
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
                "gtir" => OpCode::GtIR(a, b, c),
                "gtri" => OpCode::GtRI(a, b, c),
                "gtrr" => OpCode::GtRR(a, b, c),
                "eqir" => OpCode::EqIR(a, b, c),
                "eqri" => OpCode::EqRI(a, b, c),
                "eqrr" => OpCode::EqRR(a, b, c),
                _ => OpCode::Nop,
            };

            Some(op)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Debug)]
enum MachineState {
    Active,
    Halted,
}

struct Machine {
    registers: Registers,
    ip_reg: u8,
    program: Vec<OpCode>,
    state: MachineState,
}

impl Machine {
    fn new(ip_reg: u8, program: Vec<OpCode>) -> Machine {
        Machine {
            registers: Registers::new(),
            ip_reg,
            program,
            state: Active,
        }
    }

    fn execute_step(&mut self) {
        let ip_reg = &(self.ip_reg as i32);
        let ip = self.registers.get(ip_reg);

        if let Some(instruction) = self.program.get(ip as usize) {
            let new_registers = instruction.apply(&self.registers);
            let new_ip = new_registers.get(ip_reg);
            let new_registers = new_registers.set(ip_reg, new_ip + 1);
            self.registers = new_registers;
        } else {
            self.state = Halted;
        }
    }

    fn run(&mut self) {
        while let Active = self.state {
            self.execute_step();
        }
    }

    fn parse(input: &[&str]) -> Machine {
        let mut ip_reg = 0;
        let mut program = Vec::new();

        for line in input {
            if line.starts_with('#') {
                ip_reg = line.chars().last().unwrap().to_string().parse().unwrap();
            } else if let Some(op) = OpCode::parse(line) {
                program.push(op);
            }
        }

        Machine::new(ip_reg, program)
    }
}

fn main() {
    let start = std::time::Instant::now();

    let input = read_file("19");
    let input: Vec<&str> = input.trim().split('\n').collect();

    println!("{}", first(&input));
    println!("{}", second(&input));

    println!("elapsed {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<&str> = "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5"
            .split("\n")
            .collect();

        let mut run;
        let mut machine = Machine::parse(&input);
        assert_eq!(machine.registers.inner, (0, 0, 0, 0, 0, 0));

        run = machine.execute_step();
        assert_eq!(machine.state, MachineState::Active);
        assert_eq!(machine.registers.inner, (1, 5, 0, 0, 0, 0));

        run = machine.execute_step();
        assert_eq!(machine.state, MachineState::Active);
        assert_eq!(machine.registers.inner, (2, 5, 6, 0, 0, 0));

        run = machine.execute_step();
        assert_eq!(machine.state, MachineState::Active);
        assert_eq!(machine.registers.inner, (4, 5, 6, 0, 0, 0));

        run = machine.execute_step();
        assert_eq!(machine.state, MachineState::Active);
        assert_eq!(machine.registers.inner, (6, 5, 6, 0, 0, 0));

        run = machine.execute_step();
        assert_eq!(machine.state, MachineState::Active);
        assert_eq!(machine.registers.inner, (7, 5, 6, 0, 0, 9));

        run = machine.execute_step();
        assert_eq!(machine.state, MachineState::Halted);
    }
}
