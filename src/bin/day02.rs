extern crate itertools;

use std::fs;

use itertools::Itertools;

fn first(input: &Vec<&str>) -> u32 {
    let counts = input.iter().map(|s| { count(s) });
    let (couples, triples) = counts.fold((0u32, 0u32), |acc, (c, t)| {
        let mut couples = acc.0;
        if c { couples += 1; }

        let mut triples = acc.1;
        if t { triples += 1; }

        (couples, triples)
    });

    (couples * triples) as u32
}

fn second(input: &Vec<&str>) -> u32 {
    unimplemented!()
}

type Groups = (bool, bool);

fn count(input: &str) -> Groups {
    let groups = input.chars().sorted().group_by(|&c| { c });
    let counts: Vec<usize> = groups.into_iter().map(|(_, g)| { g.count() }).collect();
    let couples = counts.iter().any(|&x| { x == 2 });
    let triples = counts.iter().any(|&x| { x == 3 });
    (couples, triples)
}

fn main() {
    let input = read_file("02");
    let input: Vec<&str> = input.trim().split("\n").collect();

    println!("{}", first(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn abcdef() {
        assert_eq!(count("abcdef"), (false, false));
    }

    #[test]
    fn bababc() {
        assert_eq!(count("bababc"), (true, true));
    }

    #[test]
    fn abbcde() {
        assert_eq!(count("abbcde"), (true, false));
    }

    #[test]
    fn abcccd() {
        assert_eq!(count("abcccd"), (false, true));
    }

    #[test]
    fn aabcdd() {
        assert_eq!(count("aabcdd"), (true, false));
    }

    #[test]
    fn abcdee() {
        assert_eq!(count("abcdee"), (true, false));
    }

    #[test]
    fn ababab() {
        assert_eq!(count("ababab"), (false, true));
    }

    #[test]
    fn test() {
        assert_eq!(first(&vec!["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"]), 12);
    }

}

fn read_file(day: &str) -> String {
    let path = format!("input/{}.txt", day);
    fs::read_to_string(path).unwrap()
}
