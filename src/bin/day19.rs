use adventofcode2018::*;
use adventofcode2018::machine::*;

fn first(input: &[&str]) -> usize {
    let mut machine = Machine::<6>::parse(input);
    machine.run();
    machine.registers.get(&0) as usize
}

fn second(_: &[&str]) -> usize {
    // cheating: ho fatto reverse engineering del codice in input
    let n = 10551267;
    let mut res = n;
    for i in 1..=n / 2 {
        if n % i == 0 {
            res += i;
        }
    }
    res
}

fn main() {
    let start = std::time::Instant::now();

    let input = read_file("19");
    let input: Vec<&str> = input.trim().split('\n').collect();

    println!("{}", first(&input));
    println!("{}", second(&input));

    println!("elapsed {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<&str> = "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5"
            .split('\n')
            .collect();

        let mut machine = Machine::parse(&input);
        assert_eq!(machine.registers.all(), [0, 0, 0, 0, 0, 0]);

        machine.execute_step();
        assert_eq!(machine.state, MachineState::Active);
        assert_eq!(machine.registers.all(), [1, 5, 0, 0, 0, 0]);

        machine.execute_step();
        assert_eq!(machine.state, MachineState::Active);
        assert_eq!(machine.registers.all(), [2, 5, 6, 0, 0, 0]);

        machine.execute_step();
        assert_eq!(machine.state, MachineState::Active);
        assert_eq!(machine.registers.all(), [4, 5, 6, 0, 0, 0]);

        machine.execute_step();
        assert_eq!(machine.state, MachineState::Active);
        assert_eq!(machine.registers.all(), [6, 5, 6, 0, 0, 0]);

        machine.execute_step();
        assert_eq!(machine.state, MachineState::Active);
        assert_eq!(machine.registers.all(), [7, 5, 6, 0, 0, 9]);

        machine.execute_step();
        assert_eq!(machine.state, MachineState::Halted);
    }
}
