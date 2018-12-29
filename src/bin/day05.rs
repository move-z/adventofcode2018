extern crate adventofcode2018;

use adventofcode2018::*;

fn first(input: &str) -> u32 {
    strip(input.to_string()).len() as u32
}

fn second(input: &str) -> u32 {
    let stripped = strip(input.to_string());

    let subst = ('a' as u8 ..= 'z' as u8).map(|c| {
        let cl = c as char;
        let cu = cl.to_uppercase().next().unwrap();

        stripped.to_string().replace(cl, "").replace(cu, "")
    }).filter(|s| {
        s != &stripped
    }).map(|s| {
        strip(s).len()
    }).min().unwrap();

    subst as u32
}

fn strip(input: String) -> String {
    fn react(a: char, b: char) -> bool {
        a != b &&
            (a.to_lowercase().next().unwrap() == b ||
             b.to_lowercase().next().unwrap() == a)
    }

    let mut chars = input.chars().collect::<Vec<char>>();
    loop {
        let mut changed = false;
        let mut last_deleted = None;

        chars = chars.iter().enumerate().filter(|(i, &c)| {
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
        }).map(|(_, c)| { *c }).collect();

        if !changed {
            break;
        }
    }

    chars.iter().collect()
}

fn main() {
    let input = read_file("05");
    let input: &str = input.trim();

    println!("{}", first(&input));

    println!("{}", second(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(first("dabAcCaCBAcCcaDA"), 10);
    }

    #[test]
    fn test2() {
        assert_eq!(second("dabAcCaCBAcCcaDA"), 4);
    }
}
