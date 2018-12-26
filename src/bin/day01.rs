use std::fs;

fn first(input: &Vec<&str>) -> i32 {
    input.iter().map(|x| { x.parse::<i32>().unwrap() }).sum()
}

fn second(input: &Vec<&str>) -> i32 {
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
    fn test() {
        assert_eq!(first(&vec!["+1", "-2", "+3", "+1"]), 3);
    }

    #[test]
    fn test1() {
        assert_eq!(first(&vec!["+1", "+1", "+1"]), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(first(&vec!["+1", "+1", "-2"]), 0);
    }

    #[test]
    fn test3() {
        assert_eq!(first(&vec!["-1", "-2", "-3"]), -6);
    }
}

fn read_file(day: &str) -> String {
    let path = format!("input/{}.txt", day);
    fs::read_to_string(path).unwrap()
}
