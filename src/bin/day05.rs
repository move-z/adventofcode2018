extern crate adventofcode2018;

use adventofcode2018::*;

fn first(input: &str) -> u32 {
    strip(input.to_string()).len() as u32
}

fn second(input: &str) -> u32 {
    unimplemented!()
}

fn strip(input: String) -> String {
    fn react(a: char, b: char) -> bool {
        a != b &&
            (a.to_lowercase().next().unwrap() == b ||
             b.to_lowercase().next().unwrap() == a)
    }

    let mut input = input;
    loop {
        let mut changed = false;
        let chars = input.chars().collect::<Vec<char>>();
        let mut last_deleted = None;

        input = chars.iter().enumerate().filter(|(i, &c)| {
            let i = *i;

            let remove = if i > 0 && last_deleted != Some(i - 1) && react(c, chars[i - 1]) {
                true
            } else if i < chars.len() - 1 && react(c, chars[i + 1]) {
                last_deleted = Some(i + 1);
                true
            } else {
                false
            };

            changed |= remove;
            !remove
        }).map(|(_, c)| { c }).collect();

        if !changed {
            return input;
        }
    }
}

fn main() {
    let input = read_file("05");
    let input: &str = input.trim();

    println!("{}", first(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(first("dabAcCaCBAcCcaDA"), 10);
    }
}
