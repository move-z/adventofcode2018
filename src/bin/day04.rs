extern crate adventofcode2018;
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

use regex::Regex;

use adventofcode2018::*;

fn first(input: &Vec<&str>) -> u32 {
    let mut input = input.clone();
    input.sort();
    let map = parse(&input);
    let guard = map.iter().max_by_key(|g| {
        g.1.values().sum::<u32>()
    }).unwrap();
    let max_min = guard.1.iter().max_by_key(|m| { m.1 }).unwrap().0;
    *max_min as u32 * guard.0
}

fn second(input: &Vec<&str>) -> u32 {
    unimplemented!()
}

type Id = u32;
type Minute = u8;

fn parse(input: &Vec<&str>) -> HashMap<Id, HashMap<Minute, u32>> {
    let mut r: HashMap<Id, HashMap<Minute, u32>> = HashMap::new();

    let mut id = None;
    let mut start_minute = None;
    for line in input {
        if let Some(i) = parse_guard(line) {
            id = Some(i);
        } else if let Some(minute) = parse_sleep(&line) {
            start_minute = Some(minute);
        } else if let Some(end_minute) = parse_wake(&line) {
            if !r.contains_key(&id.unwrap()) {
                r.insert(id.unwrap(), HashMap::new());
            }

            let mut minutes = r.get_mut(&id.unwrap()).unwrap();
            fill_minutes(start_minute.unwrap(), end_minute, minutes);
        }
    }

    r
}

lazy_static! {
    static ref GUARD_RE: Regex = Regex::new(r"^.* Guard #(\d+) .*$").unwrap();
    static ref SLEEPS_RE: Regex = Regex::new(r"^.*:(\d{2})\] falls asleep$").unwrap();
    static ref WAKES_RE: Regex = Regex::new(r"^.*:(\d{2})\] wakes up$").unwrap();
}

fn parse_guard(line: &str) -> Option<Id> {
    if let Some(cap) = GUARD_RE.captures(line) {
        let id = parse_capture(&cap, 1, "id").unwrap();
        return Some(id);
    }
    None
}

fn parse_sleep(line: &str) -> Option<Minute> {
    if let Some(cap) = SLEEPS_RE.captures(line) {
        let minute = parse_capture(&cap, 1, "minute").unwrap();
        return Some(minute);
    }
    None
}

fn parse_wake(line: &str) -> Option<Minute> {
    if let Some(cap) = WAKES_RE.captures(line) {
        let minute = parse_capture(&cap, 1, "minute").unwrap();
        return Some(minute)
    }
    None
}

fn fill_minutes(start_minute: Minute, end_minute: Minute, minutes: &mut HashMap<Minute, u32>) {
    for minute in start_minute..end_minute {
        if minutes.contains_key(&minute) {
            let v = minutes.get_mut(&minute).unwrap();
            *v += 1;
        } else {
            minutes.insert(minute, 1);
        }
    }
}

fn main() {
    let input = read_file("04");
    let input: Vec<&str> = input.trim().split("\n").collect();

    println!("{}", first(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(first(&vec!["[1518-11-01 00:00] Guard #10 begins shift",
                               "[1518-11-01 00:05] falls asleep",
                               "[1518-11-01 00:25] wakes up",
                               "[1518-11-01 00:30] falls asleep",
                               "[1518-11-01 00:55] wakes up",
                               "[1518-11-01 23:58] Guard #99 begins shift",
                               "[1518-11-02 00:40] falls asleep",
                               "[1518-11-02 00:50] wakes up",
                               "[1518-11-03 00:05] Guard #10 begins shift",
                               "[1518-11-03 00:24] falls asleep",
                               "[1518-11-03 00:29] wakes up",
                               "[1518-11-04 00:02] Guard #99 begins shift",
                               "[1518-11-04 00:36] falls asleep",
                               "[1518-11-04 00:46] wakes up",
                               "[1518-11-05 00:03] Guard #99 begins shift",
                               "[1518-11-05 00:45] falls asleep",
                               "[1518-11-05 00:55] wakes up"]), 240);
    }
}
