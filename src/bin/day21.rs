use std::collections::HashSet;

use adventofcode2018::*;
use adventofcode2018::machine::*;

fn first(input: &[&str]) -> i32 {
    let mut machine = Machine::<6>::parse(input);
    loop {
        machine.execute_step();

        if machine.curr_ip() == 28 {
            return machine.registers.get(&4);
        }
    }
}

fn second(input: &[&str]) -> i32 {
    let mut machine = Machine::<6>::parse(input);
    let mut visited = HashSet::new();
    let mut prev = 0;
    loop {
        machine.execute_step();

        if machine.curr_ip() == 28 {
            let cur = machine.registers.get(&4);
            if visited.contains(&cur) {
                return prev;
            }
            visited.insert(cur);
            prev = cur;
        }
    }
}

fn main() {
    let start = std::time::Instant::now();

    let input = read_file("21");
    let input: Vec<&str> = input.trim().split('\n').collect();

    println!("{}", first(&input));
    println!("{}", second(&input));

    println!("elapsed {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {}
}
