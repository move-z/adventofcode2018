use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use lazy_static::lazy_static;
use regex::Regex;

use adventofcode2018::*;
use adventofcode2018::machine::*;

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

    let mut regs = Registers::new([0; 4]);
    for op in ops {
        let op = parse_op(op, &op_by_code);
        regs = op.apply(&regs);
    }
    regs.get(&0)
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

type SmallRegisters = Registers<4>;

struct Sample {
    before: SmallRegisters,
    after: SmallRegisters,
    op: (i32, i32, i32, i32),
}

lazy_static! {
    static ref REGISTERS_RE: Regex = Regex::new(r"\[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
    static ref OP_RE: Regex = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();
}

fn parse(input: &str) -> Option<SmallRegisters> {
    if let Some(cap) = REGISTERS_RE.captures(input) {
        let a = parse_capture(&cap, 1, "a").unwrap();
        let b = parse_capture(&cap, 2, "b").unwrap();
        let c = parse_capture(&cap, 3, "c").unwrap();
        let d = parse_capture(&cap, 4, "d").unwrap();
        Some(Registers::new([a, b, c, d]))
    } else {
        None
    }
}

impl Sample {
    fn parse(before: &str, after: &str, op: &str) -> Option<Sample> {
        let before = parse(before)?;
        let after = parse(after)?;

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
