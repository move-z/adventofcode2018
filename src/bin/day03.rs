use adventofcode2018::*;

use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

fn first(input: &[&str]) -> u32 {
    let claims = input.iter().map(|c| match Claim::new(c) {
        Ok(o) => o,
        Err(e) => panic!("failed to parse \"{}\": {}", c, e),
    });

    let mut visited = HashSet::new();
    let mut doubles = HashSet::new();

    for claim in claims {
        for x in claim.x..(claim.x + claim.w) {
            for y in claim.y..(claim.y + claim.h) {
                let k = (x, y);
                if !visited.insert(k) {
                    doubles.insert(k);
                }
            }
        }
    }

    doubles.len() as u32
}

fn second(input: &[&str]) -> u32 {
    let claims: Vec<Claim> = input
        .iter()
        .map(|c| match Claim::new(c) {
            Ok(o) => o,
            Err(e) => panic!("failed to parse \"{}\": {}", c, e),
        })
        .collect();

    fn overlap(a: &Claim, b: &Claim) -> bool {
        fn o(ai: usize, af: usize, bi: usize, bf: usize) -> bool {
            if ai <= bi {
                af > bi
            } else {
                o(bi, bf, ai, af)
            }
        };
        o(a.x, a.x + a.w, b.x, b.x + b.w) && o(a.y, a.y + a.h, b.y, b.y + b.h)
    }

    claims
        .iter()
        .find(|a| {
            claims
                .iter()
                .find(|&b| a.id != b.id && overlap(&a, &b))
                .is_none()
        })
        .unwrap()
        .id as u32
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
            }
            None => Err(format!("failed to parse {}", from)),
        }
    }
}

fn main() {
    let start = std::time::Instant::now();

    let input = read_file("03");
    let input: Vec<&str> = input.trim().split('\n').collect();

    println!("{}", first(&input));

    println!("{}", second(&input));

    println!("elapsed {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            4,
            first(&vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"])
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            3,
            second(&vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"])
        );
    }
}
