use adventofcode2018::*;

use regex::Regex;

use lazy_static::lazy_static;

fn first(input: &[&str]) -> usize {
    let (samples, _) = parse_file(input);
    samples.iter().filter(|s| s.opcodes().len() >= 3).count()
}

fn parse_file<'a>(input: &[&'a str]) -> (Vec<Sample>, Vec<&'a str>) {
    let mut samples = Vec::new();
    let mut ops = Vec::new();

    let mut before = "";
    let mut op= "";

    for &line in input {
        if line.starts_with("Before") {
            before = &line[8..];
        } else if line.starts_with("After") {
            let after = &line[7..];
            if let Some(sample) = Sample::parse(before, after, op) {
                samples.push(sample);
            }
            before = "";
        } else if line.len() > 0 {
            if before.len() > 0 {
                op = line;
            } else {
                ops.push(op);
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

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
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
    GtiIR(i32, i32, i32),
    GtiRI(i32, i32, i32),
    GtiRR(i32, i32, i32),
    EqIR(i32, i32, i32),
    EqRI(i32, i32, i32),
    EqRR(i32, i32, i32),
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

            OpCode::GtiIR(a, b, c) => res.set(c, if *a > res.get(b) { 1 } else { 0 }),
            OpCode::GtiRI(a, b, c) => res.set(c, if res.get(a) > *b { 1 } else { 0 }),
            OpCode::GtiRR(a, b, c) => res.set(c, if res.get(a) > res.get(b) { 1 } else { 0 }),

            OpCode::EqIR(a, b, c) => res.set(c, if *a == res.get(b) { 1 } else { 0 }),
            OpCode::EqRI(a, b, c) => res.set(c, if res.get(a) == *b { 1 } else { 0 }),
            OpCode::EqRR(a, b, c) => res.set(c, if res.get(a) == res.get(b) { 1 } else { 0 }),
        }

        res
    }

    fn test(&self, before: &Registers, after: &Registers) -> bool {
        self.apply(before) == *after
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
            Some(Sample {
                before, after, op
            })
        } else {
            None
        }
    }

    fn opcodes(&self) -> Vec<OpCode> {
        let mut res = Vec::new();
        let mut t = |o: OpCode| {
            if o.test(&self.before, &self.after) {
                res.push(o);
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
        t(OpCode::GtiIR(self.op.1, self.op.2, self.op.3));
        t(OpCode::GtiRI(self.op.1, self.op.2, self.op.3));
        t(OpCode::GtiRR(self.op.1, self.op.2, self.op.3));
        t(OpCode::EqIR(self.op.1, self.op.2, self.op.3));
        t(OpCode::EqRI(self.op.1, self.op.2, self.op.3));
        t(OpCode::EqRR(self.op.1, self.op.2, self.op.3));

        res
    }
}

fn main() {
    let start = std::time::Instant::now();

    let input = read_file("16");
    let input: Vec<&str> = input.trim().split('\n').collect();

    println!("{}", first(&input));

    println!("elapsed {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_ops() {
        let input = Sample::parse("[3, 2, 1, 1]", "[3, 2, 2, 1]", "9 2 1 2")
            .unwrap();

        assert_eq!(3, input.opcodes().len());
    }
}
