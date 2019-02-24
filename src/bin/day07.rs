extern crate adventofcode2018;
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::collections::HashSet;

use regex::Regex;

use adventofcode2018::*;

fn first(input: &Vec<&str>) -> String {
    let mut deps = parse(input);

    travel(&mut deps)
}

fn second(input: &Vec<&str>) -> usize {
    do_second(input, 5, 60)
}

fn do_second(input: &Vec<&str>, worker_num: usize, delay: usize) -> usize {
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

        let eta = workers.iter().map(|w| { w.1 }).min().unwrap();
        workers = workers.iter().map(|w| {
            let new_eta = w.1 - eta;
            if new_eta == 0 { remove(input, &w.0 ) }
            (w.0, new_eta)
        }).filter(|w| { w.1 > 0 }).collect();
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
                remove(input, &next);
            }
            None => { break; }
        }
    }

    res
}

fn next(input: &mut HashMap<char, HashSet<char>>) -> Vec<char> {
    let mut free = input.iter()
        .filter(|(_, tos)| { tos.is_empty() })
        .map(|(from, _)| { from.clone() })
        .collect::<Vec<char>>();
    free.sort();
    free
}

fn remove(input: &mut HashMap<char, HashSet<char>>, next: &char) {
    let keys = input.clone();
    for k in keys.keys() {
        let mut tos = input.get_mut(k).unwrap();
        tos.remove(next);
    }
}

lazy_static! {
     static ref RE: Regex = Regex::new(r"^Step (.) must be finished before step (.) can begin.$").unwrap();
}

fn parse(input: &Vec<&str>) -> HashMap<char, HashSet<char>> {
    let mut map: HashMap<char, HashSet<char>> = HashMap::new();

    for s in input {
        if let Some(cap) = RE.captures(s) {
            let from = parse_capture::<char>(&cap, 1, "to").unwrap();
            let to = parse_capture::<char>(&cap, 2, "from").unwrap();

            if !map.contains_key(&from) {
                map.insert(from, HashSet::new());
            }
            if !map.contains_key(&to) {
                map.insert(to, HashSet::new());
            }

            match map.get_mut(&to) {
                Some(set) => {
                    set.insert(from);
                },
                None => {
                    panic!("can't happen");
                }
            };
        }
    }

    map
}

fn main() {
    let input = read_file("07");
    let input: Vec<&str> = input.trim().split("\n").collect();

    println!("{}", first(&input));

    println!("{}", second(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(first(&vec![
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step B must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step F must be finished before step E can begin."]), "CABDFE");
    }

    #[test]
    fn test2() {
        assert_eq!(do_second(&vec![
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step B must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step F must be finished before step E can begin."], 2, 0), 15);
    }
}