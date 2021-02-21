use adventofcode2018::*;

fn first(input: &str) -> u32 {
    strip(input.chars().map(|c| c as u8).collect()).len() as u32
}

fn second(input: &str) -> u32 {
    let stripped = strip(input.chars().map(|c| c as u8).collect());

    let subst = (b'a'..=b'z')
        .map(|c| {
            stripped
                .iter()
                .filter_map(|&ch| {
                    if ch != c && ch != (c as char).to_uppercase().next().unwrap() as u8 {
                        Some(ch)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .filter(|s| s != &stripped)
        .map(|s| strip(s).len())
        .min()
        .unwrap();

    subst as u32
}

fn strip(input: Vec<u8>) -> Vec<u8> {
    fn react(a: u8, b: u8) -> bool {
        // a != b && (a.to_lowercase().next().unwrap() == b || b.to_lowercase().next().unwrap() == a)
        a != b && (a as i8 - b as i8).abs() == 32
    }

    let mut res = input;
    loop {
        let mut changed = false;
        let mut last_deleted = None;

        res = res
            .iter()
            .enumerate()
            .filter(|(i, &c)| {
                let i = *i;

                let remove = if i > 0 && last_deleted != Some(i - 1) && react(c, res[i - 1]) {
                    true
                } else if i < res.len() - 1 && react(c, res[i + 1]) {
                    last_deleted = Some(i + 1);
                    true
                } else {
                    false
                };

                changed |= remove;
                !remove
            })
            .map(|(_, c)| *c)
            .collect();

        if !changed {
            break;
        }
    }

    res
}

fn main() {
    let start = std::time::Instant::now();

    let input = read_file("05");
    let input: &str = input.trim();

    println!("{}", first(&input));

    println!("{}", second(&input));

    println!("elapsed {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(10, first("dabAcCaCBAcCcaDA"));
    }

    #[test]
    fn test2() {
        assert_eq!(4, second("dabAcCaCBAcCcaDA"));
    }
}
