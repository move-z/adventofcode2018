use adventofcode2018::*;

fn first(_input: &[&str]) -> u32 {
    unimplemented!()
}

fn main() {
    let start = std::time::Instant::now();

    let input = read_file("01");
    let input: Vec<&str> = input.trim().split('\n').collect();

    println!("{}", first(&input));

    println!("elapsed {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(first(&vec!["a", "a", "a"]), 3);
    }
}
