use adventofcode2018::*;

use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;

fn first(input: &[&str]) -> String {
    let mut deps = parse(input);

    travel(&mut deps)
}

fn second(input: &[&str]) -> usize {
    do_second(input, 5, 60)
}

fn do_second(input: &[&str], worker_num: usize, delay: usize) -> usize {
    let mut deps = parse(input);

    run(&mut deps, worker_num, delay)
}

fn run(input: &mut HashMap<char, HashSet<char>>, worker_num: usize, delay: usize) -> usize {
    let mut time = 0usize;

    let mut workers = Vec::new();

    loop {
        if input.is_empty() {
            break;
        }

        let next = next(input);

        for next in next {
            if workers.len() < worker_num {
                input.remove(&next);
                workers.push((next, next as usize - 'A' as usize + delay + 1));
            }
        }

        let eta = workers.iter().map(|w| w.1).min().unwrap();
        workers = workers
            .iter()
            .map(|w| {
                let new_eta = w.1 - eta;
                if new_eta == 0 {
                    remove(input, w.0)
                }
                (w.0, new_eta)
            })
            .filter(|w| w.1 > 0)
            .collect();
        time += eta;
    }

    time
}

fn travel(input: &mut HashMap<char, HashSet<char>>) -> String {
    let mut res = String::new();

    loop {
        let next = next(input);

        match next.first() {
            Some(next) => {
                res.push(*next);
                input.remove(next);
                remove(input, *next);
            }
            None => {
                break;
            }
        }
    }

    res
}

fn next(input: &mut HashMap<char, HashSet<char>>) -> Vec<char> {
    let mut free = input
        .iter()
        .filter(|(_, tos)| tos.is_empty())
        .map(|(from, _)| *from)
        .collect::<Vec<char>>();
    free.sort_unstable();
    free
}

fn remove(input: &mut HashMap<char, HashSet<char>>, next: char) {
    let keys = input.clone();
    for k in keys.keys() {
        let tos = input.get_mut(k).unwrap();
        tos.remove(&next);
    }
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^Step (.) must be finished before step (.) can begin.$").unwrap();
}

fn parse(input: &[&str]) -> HashMap<char, HashSet<char>> {
    let mut map: HashMap<char, HashSet<char>> = HashMap::new();

    for s in input {
        if let Some(cap) = RE.captures(s) {
            let from = parse_capture::<char>(&cap, 1, "to").unwrap();
            let to = parse_capture::<char>(&cap, 2, "from").unwrap();

            map.entry(from).or_insert_with(HashSet::new);
            map.entry(to).or_insert_with(HashSet::new);

            match map.get_mut(&to) {
                Some(set) => {
                    set.insert(from);
                }
                None => {
                    panic!("can't happen");
                }
            };
        }
    }

    map
}

fn main() {
    let start = std::time::Instant::now();

    let input = read_file("07");
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
            "CABDFE",
            first(&vec![
                "Step C must be finished before step A can begin.",
                "Step C must be finished before step F can begin.",
                "Step A must be finished before step B can begin.",
                "Step A must be finished before step D can begin.",
                "Step B must be finished before step E can begin.",
                "Step D must be finished before step E can begin.",
                "Step F must be finished before step E can begin."
            ])
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            15,
            do_second(
                &vec![
                    "Step C must be finished before step A can begin.",
                    "Step C must be finished before step F can begin.",
                    "Step A must be finished before step B can begin.",
                    "Step A must be finished before step D can begin.",
                    "Step B must be finished before step E can begin.",
                    "Step D must be finished before step E can begin.",
                    "Step F must be finished before step E can begin."
                ],
                2,
                0
            )
        );
    }
}
