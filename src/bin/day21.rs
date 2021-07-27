use adventofcode2018::*;
use crate::machine::*;

fn first(input: &[&str]) -> i32 {
    let mut machine = Machine::<6>::parse(input);
    loop {
        machine.execute_step();

        if machine.curr_ip() == 28 {
            return machine.registers.get(&4);
        }
    }
}

fn second(input: &[&str]) -> usize {
    todo!()
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
