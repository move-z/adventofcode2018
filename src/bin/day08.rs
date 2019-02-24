extern crate adventofcode2018;

use adventofcode2018::*;
use std::slice::Iter;

fn first(input: &Vec<u32>) -> u32 {
    let root = build_tree(input);

    fn travel(entry: Entry) -> u32 {
        let mut sum = entry.metadata.iter().sum();
        for c in entry.children {
            sum += travel(c);
        }
        sum
    }
    travel(root)
}

fn second(input: &Vec<u32>) -> u32 {
    unimplemented!()
}

struct Entry {
    children: Vec<Entry>,
    metadata: Vec<u32>
}

fn build_tree(input: &Vec<u32>) -> Entry {
    fn build_entry(input: &mut Iter<u32>) -> Entry {
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
    let input: Vec<u32> = input.trim().split(" ").map(|s| { s.parse().unwrap() }).collect();

    println!("{}", first(&input));

    println!("elapsed {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(first(&vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2]), 138);
    }
}
