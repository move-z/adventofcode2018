extern crate adventofcode2018;
#[macro_use]
extern crate lazy_static;

use std::collections::HashSet;

use regex::Regex;

use adventofcode2018::*;


fn first(input: &Vec<&str>) -> u32 {
    let claims = input.iter().map(|c| {
        match Claim::new(c) {
            Ok(o) => o,
            Err(e) => panic!("failed to parse \"{}\": {}", c, e),
        }
    });

    let mut visited = HashSet::new();
    let mut doubles = HashSet::new();

    for claim in claims {
        for x in claim.x..(claim.x+claim.w) {
            for y in claim.y..(claim.y+claim.h) {
                let k = (x, y);
                if !visited.insert(k) {
                    doubles.insert(k);
                }
            }
        }
    }

    doubles.len() as u32
}

fn second(input: &Vec<&str>) -> u32 {
    unimplemented!()
}

struct Claim {
    id: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
}

impl Claim {
    fn new(from: &str) -> Result<Claim, String> {
        match RE.captures(from) {
            Some(cap) => {
                let c = Claim {
                    id: parse_capture(&cap, 1, "id")?,
                    x: parse_capture(&cap, 2, "x")?,
                    y: parse_capture(&cap, 3, "y")?,
                    w: parse_capture(&cap, 4, "w")?,
                    h: parse_capture(&cap, 5, "h")?,
                };
                Ok(c)
            },
            None => Err(format!("failed to parse {}", from)),
        }
    }
}

fn main() {
    let input = read_file("03");
    let input: Vec<&str> = input.trim().split("\n").collect();

    println!("{}", first(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(first(&vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"]), 4);
    }
}

