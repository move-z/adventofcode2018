extern crate adventofcode2018;

use adventofcode2018::*;

fn first(input: &Vec<&str>) -> u32 {
    unimplemented!()
}

fn second(input: &Vec<&str>) -> u32 {
    unimplemented!()
}

fn main() {
    let input = read_file("01");
    let input: Vec<&str> = input.trim().split("\n").collect();

    println!("{}", first(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aaa() {
        assert_eq!(first(&vec!["a", "a", "a"]), 3);
    }
}
