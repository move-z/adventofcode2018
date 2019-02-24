extern crate adventofcode2018;

use adventofcode2018::*;
use std::slice::Iter;

fn first(input: &Vec<usize>) -> usize {
    let root = build_tree(input);

    fn travel(entry: &Entry) -> usize {
        let mut sum = entry.metadata.iter().sum();
        for c in &entry.children {
            sum += travel(c);
        }
        sum
    }
    travel(&root)
}

fn second(input: &Vec<usize>) -> usize {
    let root = build_tree(input);

    fn value(entry: &Entry) -> usize {
        let v =
        if entry.children.is_empty() {
            entry.metadata.iter().sum()
        } else {
            entry.metadata.iter().map(|i| {
                entry.children.get(*i-1).map(|c| { value(c) }).unwrap_or(0)
            }).sum()
        };
        v
    }
    value(&root)
}

#[derive(Debug)]
struct Entry {
    children: Vec<Entry>,
    metadata: Vec<usize>
}

fn build_tree(input: &Vec<usize>) -> Entry {
    fn build_entry(input: &mut Iter<usize>) -> Entry {
        let children_num = input.next().unwrap();
        let metadata_num = input.next().unwrap();

        let mut children = Vec::new();
        for _ in 0..*children_num {
            let e = build_entry(input);
            children.push(e);
        }

        let mut metadata = Vec::new();
        for _ in 0..*metadata_num {
            metadata.push(input.next().cloned().unwrap());
        }

        Entry { children, metadata }
    }

    build_entry(&mut input.iter())
}

fn main() {
    let start = std::time::Instant::now();

    let input = read_file("08");
    let input: Vec<usize> = input.trim().split(" ").map(|s| { s.parse().unwrap() }).collect();

    println!("{}", first(&input));

    println!("{}", second(&input));

    println!("elapsed {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(first(&vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2]), 138);
    }

    #[test]
    fn test2() {
        assert_eq!(second(&vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2]), 66);
    }
}
