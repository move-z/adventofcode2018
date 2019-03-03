use adventofcode2018::*;

use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

fn first(input: &[&str]) -> u32 {
    let map = parse(input);
    let guard = map
        .iter()
        .max_by_key(|g| g.1.values().sum::<u32>())
        .unwrap();
    let max_min = guard.1.iter().max_by_key(|m| m.1).unwrap().0;
    u32::from(*max_min) * guard.0
}

fn second(input: &[&str]) -> u32 {
    let map = parse(input);
    let guard = map
        .iter()
        .max_by_key(|g| g.1.values().max().unwrap())
        .unwrap();
    let max_min = guard.1.iter().max_by_key(|m| m.1).unwrap().0;
    u32::from(*max_min) * guard.0
}

type Id = u32;
type Minute = u8;

fn parse(input: &[&str]) -> HashMap<Id, HashMap<Minute, u32>> {
    let mut input = input.to_vec();
    input.sort();

    let mut r: HashMap<Id, HashMap<Minute, u32>> = HashMap::new();

    let mut id = None;
    let mut start_minute = None;
    for line in input {
        if let Some(i) = parse_guard(line) {
            id = Some(i);
        } else if let Some(minute) = parse_sleep(&line) {
            start_minute = Some(minute);
        } else if let Some(end_minute) = parse_wake(&line) {
            r.entry(id.unwrap()).or_insert_with(HashMap::new);

            let minutes = r.get_mut(&id.unwrap()).unwrap();
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
        return Some(minute);
    }
    None
}

fn fill_minutes(start_minute: Minute, end_minute: Minute, minutes: &mut HashMap<Minute, u32>) {
    for minute in start_minute..end_minute {
        minutes.entry(minute).and_modify(|v| *v += 1).or_insert(1);
    }
}

fn main() {
    let start = std::time::Instant::now();

    let input = read_file("04");
    let input: Vec<&str> = input.trim().split('\n').collect();

    println!("{}", first(&input));

    println!("{}", second(&input));

    println!("elapsed {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    fn data<'a>() -> Vec<&'a str> {
        vec![
            "[1518-11-01 00:00] Guard #10 begins shift",
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
            "[1518-11-05 00:55] wakes up",
        ]
    }

    #[test]
    fn test1() {
        assert_eq!(first(&data()), 240);
    }

    #[test]
    fn test2() {
        assert_eq!(second(&data()), 4455);
    }
}
