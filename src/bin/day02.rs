use adventofcode2018::*;

use itertools::Itertools;

fn first(input: &[&str]) -> u32 {
    let counts = input.iter().map(|s| count(s));
    let (couples, triples) = counts.fold((0u32, 0u32), |acc, (c, t)| {
        let mut couples = acc.0;
        if c {
            couples += 1;
        }

        let mut triples = acc.1;
        if t {
            triples += 1;
        }

        (couples, triples)
    });

    (couples * triples) as u32
}

fn second(input: &[&str]) -> String {
    fn diff(a: &str, b: &str) -> Option<String> {
        fn diff_idx(a: &str, b: &str) -> Option<usize> {
            if a.len() != b.len() {
                return None;
            }

            let mut r = None;

            for i in 0..a.len() {
                if a[i..=i] != b[i..=i] {
                    match r {
                        Some(_) => {
                            r = None;
                            break;
                        }
                        None => r = Some(i),
                    }
                }
            }

            r
        }

        let idx = diff_idx(a, b);

        let mut r = String::from(a);

        idx.map(|i| {
            r.remove(i);
            r
        })
    }

    for &a in input {
        for &b in input {
            if let Some(r) = diff(a, b) {
                return r;
            }
        }
    }

    String::from("")
}

type Groups = (bool, bool);

fn count(input: &str) -> Groups {
    let groups = input.chars().sorted().group_by(|&c| c);
    let counts: Vec<usize> = groups.into_iter().map(|(_, g)| g.count()).collect();
    let couples = counts.iter().any(|&x| x == 2);
    let triples = counts.iter().any(|&x| x == 3);
    (couples, triples)
}

fn main() {
    let start = std::time::Instant::now();

    let input = read_file("02");
    let input: Vec<&str> = input.trim().split('\n').collect();

    println!("{}", first(&input));

    println!("{}", second(&input));

    println!("elapsed {:?}", start.elapsed());
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
        assert_eq!(
            first(&vec![
                "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"
            ]),
            12
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            second(&vec![
                "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"
            ]),
            "fgij"
        )
    }
}
