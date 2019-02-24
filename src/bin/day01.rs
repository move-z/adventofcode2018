extern crate adventofcode2018;

use std::collections::HashSet;

use adventofcode2018::*;

fn first(input: &Vec<&str>) -> i32 {
    input.iter().map(|x| { x.parse::<i32>().unwrap() }).sum()
}

fn second(input: &Vec<&str>) -> i32 {
    let mut cur = 0;
    let mut visited = HashSet::new();
    visited.insert(cur);

    let input: Vec<i32> = input.iter().map(|x| { x.parse::<i32>().unwrap() }).collect();

    let mut find = || {
        input.iter().map(|x| { cur += x; cur }).find(|x| { !visited.insert(*x) })
    };

    loop {
        if let Some(i) = find() {
            return i
        }
    }
}

fn main() {
    let start = std::time::Instant::now();

    let input = read_file("01");
    let input: Vec<&str> = input.trim().split("\n").collect();

    println!("{}", first(&input));

    println!("{}", second(&input));

    println!("elapsed {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(first(&vec!["+1", "-2", "+3", "+1"]), 3);
    }

    #[test]
    fn test11() {
        assert_eq!(first(&vec!["+1", "+1", "+1"]), 3);
    }

    #[test]
    fn test12() {
        assert_eq!(first(&vec!["+1", "+1", "-2"]), 0);
    }

    #[test]
    fn test13() {
        assert_eq!(first(&vec!["-1", "-2", "-3"]), -6);
    }

    #[test]
    fn test2() {
        assert_eq!(second(&vec!["+1", "-2", "+3", "+1"]), 2);
    }

    #[test]
    fn test21() {
        assert_eq!(second(&vec!["+1", "-1"]), 0);
    }

    #[test]
    fn test22() {
        assert_eq!(second(&vec!["+3", "+3", "+4", "-2", "-4"]), 10);
    }

    #[test]
    fn test23() {
        assert_eq!(second(&vec!["-6", "+3", "+8", "+5", "-6"]), 5);
    }

    #[test]
    fn test24() {
        assert_eq!(second(&vec!["+7", "+7", "-2", "-7", "-4"]), 14);
    }
}
