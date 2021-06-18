use adventofcode2018::*;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use lazy_static::lazy_static;

fn first(input: &[&str]) -> usize {
    let (samples, _) = parse_file(input);
    samples.iter().filter(|s| s.opcodes().len() >= 3).count()
}

fn second(input: &[&str]) -> i32 {
    let (samples, ops) = parse_file(input);

    let mut ops_by_code: HashMap<i32, HashSet<OpCode>> = HashMap::new();
    for sample in samples.iter() {
        let opcode = sample.op.0;
        // println!("SCANNING OPCODE {}", opcode);
        let valid_ops = HashSet::from_iter(sample.opcodes());
        let ops_candidates = ops_by_code.get(&opcode);
        if let Some(ops_candidates) = ops_candidates {
            // println!("CURRENT CANDIDATES: {:?}", ops_candidates);
            // println!("INTERSECT TO: {:?}", valid_ops);
            let remain: HashSet<OpCode> =
                ops_candidates.intersection(&valid_ops).cloned().collect();
            // println!("CANDIDATES: {:?}", remain);
            ops_by_code.insert(opcode, remain);
        } else {
            // println!("CANDIDATES: {:?}", valid_ops);
            ops_by_code.insert(opcode, valid_ops);
        }
    }

    loop {
        let mut found = false;
        for (sure_numcode, sure_opcode) in ops_by_code.clone().iter().filter(|(_, c)| c.len() == 1)
        {
            let sure_opcode = sure_opcode.iter().next().unwrap();
            for (numcode, opcodes) in ops_by_code.iter_mut() {
                if numcode != sure_numcode {
                    found |= opcodes.remove(sure_opcode);
                }
            }
        }
        if !found {
            break;
        }
    }

    let mut op_by_code = [OpCode::Nop; 16];
    for code in 0..16 {
        if let Some(ops) = ops_by_code.get(&code) {
            assert_eq!(ops.len(), 1);
            op_by_code[code as usize] = *ops.iter().next().unwrap();
        }
    }

    let mut regs = Registers::new(0, 0, 0, 0);
    for op in ops {
        let op = parse_op(op, &op_by_code);
        regs = op.apply(&regs);
    }
    regs.inner.0
}

fn parse_file<'a>(input: &[&'a str]) -> (Vec<Sample>, Vec<&'a str>) {
    let mut samples = Vec::new();
    let mut ops = Vec::new();

    let mut before = "";
    let mut op = "";

    for &line in input {
        if line.starts_with("Before") {
            before = &line[8..];
        } else if line.starts_with("After") {
            let after = &line[7..];
            if let Some(sample) = Sample::parse(before, after, op) {
                samples.push(sample);
            }
            before = "";
        } else if !line.is_empty() {
            if !before.is_empty() {
                op = line;
            } else {
                ops.push(line);
            }
        }
    }

    (samples, ops)
}

#[derive(Clone, PartialEq)]
struct Registers {
    inner: (i32, i32, i32, i32),
}

lazy_static! {
    static ref REGISTERS_RE: Regex = Regex::new(r"\[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
}

impl Registers {
    fn new(a: i32, b: i32, c: i32, d: i32) -> Registers {
        Registers {
            inner: (a, b, c, d),
        }
    }

    fn parse(input: &str) -> Option<Registers> {
        if let Some(cap) = REGISTERS_RE.captures(input) {
            let a = parse_capture(&cap, 1, "a").unwrap();
            let b = parse_capture(&cap, 2, "b").unwrap();
            let c = parse_capture(&cap, 3, "c").unwrap();
            let d = parse_capture(&cap, 4, "d").unwrap();
            Some(Registers {
                inner: (a, b, c, d),
            })
        } else {
            None
        }
    }

    fn get(&self, i: &i32) -> i32 {
        match i {
            0 => self.inner.0,
            1 => self.inner.1,
            2 => self.inner.2,
            3 => self.inner.3,
            _ => 0,
        }
    }

    fn set(&mut self, i: &i32, val: i32) {
        match i {
            0 => self.inner.0 = val,
            1 => self.inner.1 = val,
            2 => self.inner.2 = val,
            3 => self.inner.3 = val,
            _ => {}
        };
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
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
    GtIr(i32, i32, i32),
    GtRi(i32, i32, i32),
    GtRr(i32, i32, i32),
    EqIr(i32, i32, i32),
    EqRi(i32, i32, i32),
    EqRr(i32, i32, i32),
    Nop,
}

impl OpCode {
    fn apply(&self, input: &Registers) -> Registers {
        let mut res = input.clone();
        match self {
            OpCode::AddR(a, b, c) => res.set(c, res.get(a) + res.get(b)),
            OpCode::AddI(a, b, c) => res.set(c, res.get(a) + b),

            OpCode::MulR(a, b, c) => res.set(c, res.get(a) * res.get(b)),
            OpCode::MulI(a, b, c) => res.set(c, res.get(a) * b),

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

    fn get_default(&self) -> OpCode {
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
}

struct Sample {
    before: Registers,
    after: Registers,
    op: (i32, i32, i32, i32),
}

lazy_static! {
    static ref OP_RE: Regex = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();
}

impl Sample {
    fn parse(before: &str, after: &str, op: &str) -> Option<Sample> {
        let before = Registers::parse(before)?;
        let after = Registers::parse(after)?;

        if let Some(cap) = OP_RE.captures(op) {
            let o = parse_capture(&cap, 1, "o").unwrap();
            let a = parse_capture(&cap, 2, "a").unwrap();
            let b = parse_capture(&cap, 3, "b").unwrap();
            let c = parse_capture(&cap, 4, "c").unwrap();
            let op = (o, a, b, c);
            Some(Sample { before, after, op })
        } else {
            None
        }
    }

    fn opcodes(&self) -> Vec<OpCode> {
        let mut res = Vec::new();
        let mut t = |o: OpCode| {
            if o.apply(&self.before) == self.after {
                res.push(o.get_default());
            }
        };
        t(OpCode::AddR(self.op.1, self.op.2, self.op.3));
        t(OpCode::AddI(self.op.1, self.op.2, self.op.3));
        t(OpCode::MulR(self.op.1, self.op.2, self.op.3));
        t(OpCode::MulI(self.op.1, self.op.2, self.op.3));
        t(OpCode::BanR(self.op.1, self.op.2, self.op.3));
        t(OpCode::BanI(self.op.1, self.op.2, self.op.3));
        t(OpCode::BorR(self.op.1, self.op.2, self.op.3));
        t(OpCode::BorI(self.op.1, self.op.2, self.op.3));
        t(OpCode::SetR(self.op.1, self.op.2, self.op.3));
        t(OpCode::SetI(self.op.1, self.op.2, self.op.3));
        t(OpCode::GtIr(self.op.1, self.op.2, self.op.3));
        t(OpCode::GtRi(self.op.1, self.op.2, self.op.3));
        t(OpCode::GtRr(self.op.1, self.op.2, self.op.3));
        t(OpCode::EqIr(self.op.1, self.op.2, self.op.3));
        t(OpCode::EqRi(self.op.1, self.op.2, self.op.3));
        t(OpCode::EqRr(self.op.1, self.op.2, self.op.3));

        res
    }
}

fn parse_op(input: &str, codemap: &[OpCode; 16]) -> OpCode {
    if let Some(cap) = OP_RE.captures(input) {
        let o: usize = parse_capture(&cap, 1, "o").unwrap();
        let a = parse_capture(&cap, 2, "a").unwrap();
        let b = parse_capture(&cap, 3, "b").unwrap();
        let c = parse_capture(&cap, 4, "c").unwrap();
        match codemap[o] {
            OpCode::AddR(_, _, _) => OpCode::AddR(a, b, c),
            OpCode::AddI(_, _, _) => OpCode::AddI(a, b, c),
            OpCode::MulR(_, _, _) => OpCode::MulR(a, b, c),
            OpCode::MulI(_, _, _) => OpCode::MulI(a, b, c),
            OpCode::BanR(_, _, _) => OpCode::BanR(a, b, c),
            OpCode::BanI(_, _, _) => OpCode::BanI(a, b, c),
            OpCode::BorR(_, _, _) => OpCode::BorR(a, b, c),
            OpCode::BorI(_, _, _) => OpCode::BorI(a, b, c),
            OpCode::SetR(_, _, _) => OpCode::SetR(a, b, c),
            OpCode::SetI(_, _, _) => OpCode::SetI(a, b, c),
            OpCode::GtIr(_, _, _) => OpCode::GtIr(a, b, c),
            OpCode::GtRi(_, _, _) => OpCode::GtRi(a, b, c),
            OpCode::GtRr(_, _, _) => OpCode::GtRr(a, b, c),
            OpCode::EqIr(_, _, _) => OpCode::EqIr(a, b, c),
            OpCode::EqRi(_, _, _) => OpCode::EqRi(a, b, c),
            OpCode::EqRr(_, _, _) => OpCode::EqRr(a, b, c),
            OpCode::Nop => OpCode::Nop,
        }
    } else {
        OpCode::Nop
    }
}

fn main() {
    let start = std::time::Instant::now();

    let input = read_file("16");
    let input: Vec<&str> = input.trim().split('\n').collect();

    println!("{}", first(&input));
    println!("{}", second(&input));

    println!("elapsed {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_ops() {
        let input = Sample::parse("[3, 2, 1, 1]", "[3, 2, 2, 1]", "9 2 1 2").unwrap();

        assert_eq!(3, input.opcodes().len());
    }
}
